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

namespace Sustenet.Master
{
    using Network;
    using System.Collections.Generic;
    using Transport;
    using Transport.Messages;

    /// <summary>
    /// The Master Server keeps track of all Cluster Servers. It also allocates
    /// connecting users to Cluster Servers automatically, or allows the users
    /// to manually select one.
    /// </summary>
    class MasterServer : BaseServer
    {
        private delegate void PacketHandler(int fromClient, Packet packet);
        private static Dictionary<int, PacketHandler> packetHandlers;

        public Dictionary<int, BaseClient> clusterClients = new Dictionary<int, BaseClient>();
        public List<int> releasedClusterIds = new List<int>();

        public MasterServer(int _maxConnections = 0, ushort _port = 6256) : base(_maxConnections, _port)
        {
            InitializeData();

            Start(ServerType.MasterServer);
        }

        private void InitializeData()
        {
            if(packetHandlers == null)
            {
                packetHandlers = new Dictionary<int, PacketHandler>()
                {
                    { (int)ClientPackets.cluster, this.ValidateCluster },
                    { (int)ClientPackets.login, this.ValidateUser }
                };
            }
        }

        internal void ClearClusterClient(int clientId)
        {
            clusterClients.Remove(clientId);
            releasedClusterIds.Add(clientId);
            onDebug.RaiseEvent($"Disconnected ClusterClient#{clientId}.");
        }
    }
}
