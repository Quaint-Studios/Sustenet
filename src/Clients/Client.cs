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

namespace Sustenet.Clients
{
    using System.Net;
    using System.Collections.Generic;
    using Transport;
    using Network;


    /// <summary>
    /// A standard client that connects to a server.
    /// </summary>
    public class Client : BaseClient
    {
        public IPAddress ip;
        public ushort port;

        private Packet receivedData;

        protected delegate string PacketHandler(Packet packet);
        protected static Dictionary<int, PacketHandler> packetHandlers;

        public Client(string _ip = "127.0.0.1", ushort _port = 6256) : base(0)
        {
            ip = IPAddress.Parse(_ip);
            port = _port;

            tcp.onConnected.Run += () => {
                receivedData = new Packet();
            };

            tcp.onReceived.Run += (data) => {
                receivedData.Reset(HandleData(data));
            };

            InitializeClientData();

            tcp.Connect(ip, port);
        }

        #region Data Functions
        private bool HandleData(byte[] data)
        {
            int packetLength = 0;

            receivedData.SetBytes(data);

            if(receivedData.UnreadLength() >= 4)
            {
                packetLength = receivedData.ReadInt();
                if(packetLength <= 0)
                {
                    return true;
                }
            }

            while(packetLength > 0 && packetLength <= receivedData.UnreadLength())
            {
                byte[] packetBytes = receivedData.ReadBytes(packetLength);

                ThreadManager.ExecuteOnMainThread(() => {
                    using(Packet packet = new Packet(packetBytes))
                    {
                        int packetId = packet.ReadInt();

                        packetHandlers[packetId](packet);
                    }
                });

                packetLength = 0;

                if(receivedData.UnreadLength() >= 4)
                {
                    packetLength = receivedData.ReadInt();
                    if(packetLength <= 0)
                    {
                        return true;
                    }
                }
            }

            if(packetLength <= 1)
            {
                return true;
            }

            return false;
        }

        protected virtual void InitializeClientData()
        {
            packetHandlers = new Dictionary<int, PacketHandler>()
            {
                { (int)ServerPackets.welcome, this.Welcome }
            };
        }
        #endregion
    }
}
