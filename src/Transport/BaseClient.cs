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

    class BaseClient
    {
        public int id;
        public TcpHandler tcp;
        public static int bufferSize = 4096;

        public BaseClient(int _id)
        {
            id = _id;
            tcp = new TcpHandler(id);
        }

        public class TcpHandler
        {
            private readonly int id;

            public TcpClient socket;
            private NetworkStream stream;
            private byte[] receiveBuffer;

            public TcpHandler(int _id)
            {
                id = _id;
            }

            public void Receive(TcpClient _socket)
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

                stream.BeginRead(receiveBuffer, 0, bufferSize, ReceiveCallback, null);
            }

            private void ReceiveCallback(IAsyncResult ar)
            {
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

                    stream.BeginRead(receiveBuffer, 0, bufferSize, ReceiveCallback, null);
                }
                catch(Exception e)
                {
                    DebugClient($"Error with receiving TCP data...: {e}");
                }
            }

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

                socket.BeginConnect(ip, port, ConnectCallback, null);
            }

            private void ConnectCallback(IAsyncResult ar)
            {
                try
                {
                    socket.EndConnect(ar);

                    if(!socket.Connected)
                    {
                        DebugClient($"Failed to connect to the server at {socket.Client.RemoteEndPoint}.");
                        return;
                    }

                    DebugClient($"Connected to server at {socket.Client.RemoteEndPoint}.");

                    if(stream == null)
                    {
                        stream = socket.GetStream();
                    }

                    stream.BeginRead(receiveBuffer, 0, bufferSize, ReceiveCallback, null);
                }
                catch
                {
                    DebugClient("Error while trying to connect.");
                }
            }
        }

        protected static void DebugClient(string msg)
        {
            Console.WriteLine($"Client: {msg}");
        }
    }
}
