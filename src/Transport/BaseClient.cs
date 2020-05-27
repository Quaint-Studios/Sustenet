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
    using System;
    using System.Net;
    using System.Net.Sockets;
    using Network;
    using Events;

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

        public BaseClient(int _id, bool debug = true)
        {
            id = _id;
            tcp = new TcpHandler();
            udp = new UdpHandler();

            if(debug)
                tcp.onDebug.Run += (msg) => DebugClient(id, msg);

            udp.onDebug = tcp.onDebug;
        }

        /// <summary>
        /// Handles events for connecting, receiving, and debugging.
        /// Also controls the socket connection.
        /// </summary>
        public class TcpHandler
        {
            internal TcpClient socket;
            internal NetworkStream stream;
            private byte[] receiveBuffer;

            public BaseEvent onConnected = new BaseEvent();
            public BaseEvent onDisconnected = new BaseEvent();
            public BaseEvent<byte[]> onReceived = new BaseEvent<byte[]>();
            public BaseEvent<string> onDebug = new BaseEvent<string>();

            #region Connection Functions
            /// <summary>
            /// Used for servers that create local records of clients.
            /// It will wipe any existing connections and start a new one.
            /// </summary>
            /// <param name="_socket">The socket to replace the current socket with.</param>
            public void Receive(TcpClient _socket)
            {
                try
                {
                    if(socket != null)
                    {
                        if(stream != null)
                        {
                            stream.Close();
                        }

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

                    stream.BeginRead(receiveBuffer, 0, bufferSize, new AsyncCallback(ReceiveCallback), null);
                }
                catch
                {
                    onDisconnected.RaiseEvent();
                }
            }

            /// <summary>
            /// When the current stream receives data.
            /// </summary>
            /// <param name="ar">The result of BeginRead().</param>
            public void ReceiveCallback(IAsyncResult ar)
            {
                try
                {
                    int byteLength = stream.EndRead(ar);
                    if(byteLength <= 0)
                    {
                        // disconnect
                        onDisconnected.RaiseEvent();
                        return;
                    }

                    byte[] data = new byte[byteLength];

                    Array.Copy(receiveBuffer, data, byteLength);

                    onReceived.RaiseEvent(data);

                    stream.BeginRead(receiveBuffer, 0, bufferSize, new AsyncCallback(ReceiveCallback), null);
                }
                catch
                {
                    // onDebug.RaiseEvent($"Error with receiving TCP data...: {e}");
                    onDisconnected.RaiseEvent();
                }
            }

            /// <summary>
            /// Connects to a server.
            /// </summary>
            /// <param name="ip">The IP address.</param>
            /// <param name="port">The port number.</param>
            public void Connect(IPAddress ip, ushort port)
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

                socket.BeginConnect(ip, port, new AsyncCallback(ConnectCallback), null);
            }

            /// <summary>
            /// Triggered after BeginConnect().
            /// </summary>
            /// <param name="ar">Result from BeginConnect().</param>
            public void ConnectCallback(IAsyncResult ar)
            {
                try
                {
                    socket.EndConnect(ar);

                    if(!socket.Connected)
                    {
                        onDebug.RaiseEvent($"Failed to connect to the server at {socket.Client.RemoteEndPoint}.");
                        return;
                    }

                    onDebug.RaiseEvent($"Connected to server at {socket.Client.RemoteEndPoint}.");

                    if(stream == null)
                    {
                        stream = socket.GetStream();
                    }

                    onConnected.RaiseEvent();

                    stream.BeginRead(receiveBuffer, 0, bufferSize, new AsyncCallback(ReceiveCallback), null);
                }
                catch
                {
                    onDebug.RaiseEvent("Error while trying to connect.");
                }
            }
            #endregion
        }

        public class UdpHandler
        {
            public UdpClient socket;
            public IPEndPoint endPoint;

            public BaseEvent onConnected = new BaseEvent();
            public BaseEvent onDisconnected = new BaseEvent();
            public BaseEvent<byte[]> onReceived = new BaseEvent<byte[]>();
            public BaseEvent<string> onDebug;

            /// <summary>
            /// Prepares for a UDP connection to a server.
            /// </summary>
            /// <param name="ip">The IP Address to set the endpoint to.</param>
            /// <param name="port">The port to set the endpoint to.</param>
            /// <param name="localPort">The local port.</param>
            public void Connect(IPAddress ip, ushort port, ushort localPort)
            {
                endPoint = new IPEndPoint(ip, port);

                socket = new UdpClient(localPort);

                socket.Connect(endPoint);
                socket.BeginReceive(new AsyncCallback(ReceiveCallback), null);
            }

            private void ReceiveCallback(IAsyncResult ar)
            {
                try
                {
                    byte[] data = socket.EndReceive(ar, ref endPoint);
                    socket.BeginReceive(ReceiveCallback, null);

                    if(data.Length < 4)
                    {
                        onDisconnected.RaiseEvent();
                        return;
                    }
                }
                catch
                {

                }
            }
        }

        private static void DebugClient(int id, string msg)
        {
            Console.WriteLine($"(Client#{id}) {msg}");
        }

        private bool disposed = true;

        protected virtual void Dispose(bool disposing)
        {
            if(!disposed)
            {
                if(disposing)
                {
                    if(tcp.socket != null)
                        tcp.socket.Close();
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
