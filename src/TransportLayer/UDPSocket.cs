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

namespace Sustenet.TransportLayer
{
    using System;
    using System.Net.Sockets;
    using System.Net;

    class UDPSocket
    {
        private readonly Socket socket = new Socket(AddressFamily.InterNetwork, SocketType.Dgram, ProtocolType.Udp);

        private const ushort bufferSize = 8 * 1024;
        private readonly State state = new State();
        private const byte offset = 0;
        private EndPoint remoteEP;
        private AsyncCallback cb = null;

        class State
        {
            public byte[] buffer = new byte[bufferSize];
        }

        public UDPSocket(ushort port)
        {
            remoteEP = new IPEndPoint(IPAddress.Any, port);
        }

        public void Receive()
        {
            cb = (data) =>
            {
                State cbState = (State)data.AsyncState;
                int bytes = socket.EndReceiveFrom(data, ref remoteEP);
                socket.BeginReceiveFrom(cbState.buffer, offset, bufferSize, SocketFlags.None, ref remoteEP, cb, cbState);

                Console.WriteLine("RECV: {0}: {1}, {2}", remoteEP.ToString(), bytes, System.Text.Encoding.ASCII.GetString(cbState.buffer, 0, bytes));
            };

            socket.BeginReceiveFrom(state.buffer, offset, bufferSize, SocketFlags.None, ref remoteEP, cb, state);
        }

        public void BindAndReceive(ushort port)
        {
            BindAndReceive(IPAddress.Loopback, port);
        }
        public void BindAndReceive(string address, ushort port)
        {
            if (!IPAddress.TryParse(address, out IPAddress ip))
            {
                Console.Error.WriteLine($"Failed to bind the IP address {address}");
                return;
            }

            BindAndReceive(ip, port);
        }
        public void BindAndReceive(IPAddress address, ushort port)
        {
            socket.Bind(new IPEndPoint(address, port));
            Receive();
        }

        public void ConnectAndReceive(string address, ushort port)
        {
            socket.Connect(IPAddress.Parse(address), port);
        }
    }
}
