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
    using System;
    using System.Net;
    using System.Collections.Generic;
    using Transport;
    using Network;


    /// <summary>
    /// A standard client that connects to a server.
    /// </summary>
    public class Client : BaseClient
    {
        public enum ConnectionType
        {
            MasterServer,
            ClusterServer
        }

        public struct Connection
        {
            private IPAddress ip;
            private ushort port;
            public string Ip
            {
                get
                {
                    if(ip == null)
                    {
                        throw new Exception("Failed to get the IP address because it's not set.");
                    }
                    return ip.ToString();
                }
                set
                {
                    if(!IPAddress.TryParse(value, out ip))
                    {
                        throw new Exception("Failed to set the IP address because of an invalid format.");
                    }
                }
            }

            public ushort Port
            {
                get { return port; }
                set { port = value; }
            }
        }

        private Connection masterConnection;
        private Connection clusterConnection;

        private Packet receivedData;

        protected delegate string PacketHandler(Packet packet);
        protected static Dictionary<int, PacketHandler> packetHandlers;

        public Client(string _ip = "127.0.0.1", ushort _port = 6256, bool debug = true) : base(0, debug)
        {
            masterConnection = new Connection
            {
                Ip = _ip,
                Port = _port
            };

            tcp.onConnected.Run += () => {
                receivedData = new Packet();
            };

            tcp.onReceived.Run += (data) => {
                receivedData.Reset(HandleData(data));
            };

            InitializeClientData();
        }

        /// <summary>
        /// Connects to the currently assigned IP and port.
        /// </summary>
        public void Connect(ConnectionType connectType = ConnectionType.MasterServer)
        {
            switch(connectType)
            {
                case ConnectionType.MasterServer:
                    tcp.Connect(IPAddress.Parse(masterConnection.Ip), masterConnection.Port);
                    break;

                case ConnectionType.ClusterServer:
                    tcp.Connect(IPAddress.Parse(clusterConnection.Ip), clusterConnection.Port);
                    break;
            }
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
            if(packetHandlers == null)
            {
                packetHandlers = new Dictionary<int, PacketHandler>()
                {
                    { (int)ServerPackets.welcome, this.Welcome }
                };
            }
        }
        #endregion
    }
}
