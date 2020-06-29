/**
 * Copyright (C) 2020 Quaint Studios, Kristopher Ali (Makosai) <kristopher.ali.dev@gmail.com>
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU Affero General Public License as
 * published by the Free Software Foundation, either version 3 of the
 * License, or (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU Affero General Public License for more details.
 *
 * You should have received a copy of the GNU Affero General Public License
 * along with this program.  If not, see <http://www.gnu.org/licenses/>.
 */

namespace Sustenet.Transport
{
    using Events;
    using Network;
    using System;
    using System.Net;
    using System.Net.Sockets;
    using Utils;

    /// <summary>
    /// The core for all clients. Handles basic functionality like sending
    /// and receiving data. Also handles the core for connecting to servers.
    /// </summary>
    public class BaseClient : IDisposable
    {
        public int id;
        public TcpHandler tcp;
        public UdpHandler udp;
        public static int bufferSize = 4096;

        public string name;

        internal Packet receivedData;

        public BaseEvent onConnected = new BaseEvent();
        public BaseEvent onDisconnected = new BaseEvent();
        public BaseEvent<Protocols, byte[]> onReceived = new BaseEvent<Protocols, byte[]>();

        public BaseClient(int _id)
        {
            id = _id;

            tcp = new TcpHandler();
            udp = new UdpHandler();
        }

        /// <summary>
        /// Handles events for connecting, receiving, and debugging.
        /// Also controls the socket connection.
        /// </summary>
        public class TcpHandler : IDisposable
        {
            internal TcpClient socket;
            internal NetworkStream stream;
            private byte[] receiveBuffer;

            #region Connection Functions
            /// <summary>
            /// Used for servers that create local records of clients.
            /// It will wipe any existing connections and start a new one.
            /// </summary>
            /// <param name="_socket">The socket to replace the current socket with.</param>
            public void Receive(BaseClient client, TcpClient _socket)
            {
                try
                {
                    if(socket != null)
                    {
                        socket.Close();
                    }

                    socket = _socket;
                    socket.ReceiveBufferSize = bufferSize;
                    socket.SendBufferSize = bufferSize;

                    if(stream == null)
                    {
                        stream = socket.GetStream();
                    }

                    if(receiveBuffer == null)
                    {
                        receiveBuffer = new byte[bufferSize];
                    }

                    stream.BeginRead(receiveBuffer, 0, bufferSize, ReceiveCallback, client);
                }
                catch
                {
                    client.onDisconnected.RaiseEvent();
                }
            }

            /// <summary>
            /// When the current stream receives data.
            /// </summary>
            /// <param name="ar">The result of BeginRead().</param>
            public void ReceiveCallback(IAsyncResult ar)
            {
                BaseClient client = (BaseClient)ar.AsyncState;

                try
                {
                    int byteLength;

                    if(stream == null)
                        return;

                    byteLength = stream.EndRead(ar);

                    if(byteLength <= 0)
                    {
                        // disconnect
                        client.onDisconnected.RaiseEvent();
                        return;
                    }

                    byte[] data = new byte[byteLength];

                    Array.Copy(receiveBuffer, data, byteLength);

                    client.onReceived.RaiseEvent(Protocols.TCP, data);

                    if(stream != null)
                        stream.BeginRead(receiveBuffer, 0, bufferSize, ReceiveCallback, client);
                }
                catch(Exception e)
                {
                    Utilities.WriteLine(e);
                    // onDebug.RaiseEvent($"Error with receiving TCP data...: {e}");
                    client.onDisconnected.RaiseEvent();
                }
            }

            /// <summary>
            /// Connects to a server.
            /// </summary>
            /// <param name="ip">The IP address.</param>
            /// <param name="port">The port number.</param>
            public void Connect(BaseClient client, IPAddress ip, ushort port)
            {
                try
                {
                    if(socket == null)
                    {
                        socket = new TcpClient
                        {
                            ReceiveBufferSize = bufferSize,
                            SendBufferSize = bufferSize
                        };
                    }

                    if(receiveBuffer == null)
                    {
                        receiveBuffer = new byte[bufferSize];
                    }

                    socket.BeginConnect(ip, port, ar => ConnectCallback(ar, ip), client);
                }
                catch
                {
                    client.onDisconnected.RaiseEvent();
                }
            }

            /// <summary>
            /// Triggered after BeginConnect().
            /// </summary>
            /// <param name="ar">Result from BeginConnect().</param>
            public void ConnectCallback(IAsyncResult ar, IPAddress ip)
            {
                BaseClient client = (BaseClient)ar.AsyncState;

                try
                {
                    if(socket != null)
                        socket.EndConnect(ar);

                    if(!socket.Connected)
                    {
                        DebugClient(client.id, $"Failed to connect to the server at {socket.Client.RemoteEndPoint}.");
                        return;
                    }

                    DebugClient(client.id, $"Connected to server at {socket.Client.RemoteEndPoint}.");

                    if(stream == null)
                    {
                        stream = socket.GetStream();
                    }

                    IPEndPoint endpoint = ((IPEndPoint)socket.Client.RemoteEndPoint);

                    client.udp.Connect(client, ip, (ushort)((IPEndPoint)socket.Client.RemoteEndPoint).Port, (ushort)((IPEndPoint)socket.Client.LocalEndPoint).Port);

                    stream.BeginRead(receiveBuffer, 0, bufferSize, ReceiveCallback, client);
                }
                catch(Exception e)
                {
                    Utilities.WriteLine(e);
                    DebugClient(client.id, "Error while trying to connect via TCP.");
                }
            }
            #endregion

            private bool disposed;

            protected virtual void Dispose(bool disposing)
            {
                if(!disposed)
                {
                    if(disposing)
                    {
                        if(socket != null)
                            socket.Close();
                    }

                    disposed = true;
                }
            }

            public void Dispose()
            {
                Dispose(true);
                GC.SuppressFinalize(this);
            }
        }

        public class UdpHandler : IDisposable
        {
            public static UdpClient socket;
            public IPEndPoint endpoint;

            /// <summary>
            /// Prepares a client for a UDP connection to a server.
            /// </summary>
            /// <param name="ip">The IP Address to set the endpoint to.</param>
            /// <param name="port">The port to set the endpoint to.</param>
            /// <param name="localPort">The local port.</param>
            public void Connect(BaseClient client, IPAddress ip, ushort port, ushort localPort)
            {
                try
                {
                    endpoint = new IPEndPoint(ip, port);

                    if(socket == null)
                        socket = new UdpClient(localPort);

                    socket.Connect(endpoint);
                    socket.BeginReceive(ReceiveCallback, client);

                    client.onConnected.RaiseEvent();
                }
                catch(Exception e)
                {
                    Utilities.WriteLine(e);
                    client.onDisconnected.RaiseEvent(); // TODO: Pass a TypeEnum.UDP enum to differentiate instructions?
                }
            }

            private void ReceiveCallback(IAsyncResult ar)
            {
                BaseClient client = (BaseClient)ar.AsyncState;

                try
                {
                    byte[] data = socket.EndReceive(ar, ref endpoint);
                    socket.BeginReceive(ReceiveCallback, client);

                    if(data.Length < 4)
                    {
                        client.onDisconnected.RaiseEvent();
                        return;
                    }

                    client.onReceived.RaiseEvent(Protocols.UDP, data);
                }
                catch(Exception e)
                {
                    Utilities.WriteLine(e);
                    client.onDisconnected.RaiseEvent();
                }
            }

            private bool disposed;

            protected virtual void Dispose(bool disposing)
            {
                if(!disposed)
                {
                    if(disposing)
                    {
                        // Managed resources
                    }

                    // Unmanaged resources

                    disposed = true;
                }
            }

            public void Dispose()
            {
                Dispose(true);
                GC.SuppressFinalize(this);
            }
        }

        public static void DebugClient(int id, string msg)
        {
            Utilities.WriteLine($"(Client#{id}) {msg}");
        }

        private bool disposed;

        protected virtual void Dispose(bool disposing)
        {
            if(!disposed)
            {
                if(disposing)
                {
                    if(tcp != null)
                        tcp.Dispose();

                    if(udp != null)
                        udp.Dispose();

                    if(receivedData != null)
                        receivedData.Dispose();
                }

                disposed = true;
            }
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
    }
}
