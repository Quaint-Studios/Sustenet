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

    public class BaseClient
    {
        public int id;
        public TcpHandler tcp;
        public static int bufferSize = 4096;

        public class Dbug
        {
            public int constructed, tcpconstructed, connected, received, senddata, debugClient, receivedCB, connectedCB, initialized = 0;
        }

        public static Dbug dbug = new Dbug();

        public BaseClient(int _id, bool debug = true)
        {
            id = _id;
            tcp = new TcpHandler(id);

            if(debug)
                tcp.onDebug.Run += (msg) => DebugClient(msg);

            dbug.constructed++;
        }

        public class TcpHandler
        {
            private readonly int id;

            public TcpClient socket;
            private NetworkStream stream;
            private byte[] receiveBuffer;

            public BaseEvent onConnected = new BaseEvent();
            public BaseEvent<byte[]> onReceived = new BaseEvent<byte[]>();
            public BaseEvent<string> onDebug = new BaseEvent<string>();

            public TcpHandler(int _id)
            {
                id = _id;

                dbug.tcpconstructed++;
            }

            #region Connection Functions
            public void Receive(TcpClient _socket)
            {
                dbug.received++;

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

            public void ReceiveCallback(IAsyncResult ar)
            {
                dbug.receivedCB++;

                onDebug.RaiseEvent("RECEIVED DATA");

                try
                {
                    int byteLength = stream.EndRead(ar);
                    if(byteLength <= 0)
                    {
                        // disconnect
                        return;
                    }

                    byte[] data = new byte[byteLength];

                    Array.Copy(receiveBuffer, data, byteLength);

                    onReceived.RaiseEvent(data);

                    stream.BeginRead(receiveBuffer, 0, bufferSize, new AsyncCallback(ReceiveCallback), null);
                }
                catch(Exception e)
                {
                    onDebug.RaiseEvent($"Error with receiving TCP data...: {e}");
                }
            }

            public void Connect(IPAddress ip, ushort port)
            {
                dbug.connected++;

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

            public void ConnectCallback(IAsyncResult ar)
            {
                dbug.connectedCB++;

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

            #region Data Functions
            public void SendData(Packet packet)
            {
                dbug.senddata++;

                try
                {
                    if(socket == null)
                    {
                        throw new Exception("TCPHandler socket is null.");
                    }

                    stream.BeginWrite(packet.ToArray(), 0, packet.Length(), null, null);
                }
                catch(Exception e)
                {
                    onDebug.RaiseEvent($"Error sending data via TCP to Client#{id}...: {e}");
                }
            }
            #endregion
        }

        private static void DebugClient(string msg)
        {
            dbug.debugClient++;
            Console.WriteLine($"Client: {msg}");
        }
    }
}
