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
    using Transport.Messages.MasterHandlers;
    using Utils.Security;

    /// <summary>
    /// The Master Server keeps track of all Cluster Servers. It also allocates
    /// connecting users to Cluster Servers automatically, or allows the users
    /// to manually select one.
    /// </summary>
    class MasterServer : BaseServer
    {
        /// <summary>
        /// A list of clients that have been registered as cluster clients.
        /// </summary>
        public List<int> clusterIds = new List<int>();

        public MasterServer(int _maxConnections = 0, ushort _port = 6256) : base(ServerType.MasterServer, _maxConnections, _port)
        {
            RSAManager.LoadPubKeys();
            AESManager.LoadKeys();

            InitializeData();

            Start(ServerType.MasterServer);
        }

        private void InitializeData()
        {
            if(packetHandlers == null)
            {
                packetHandlers = new Dictionary<int, PacketHandler>()
                {
                    { (int)ClientPackets.answerPassphrase, this.AnswerPassphrase },
                    { (int)ClientPackets.validateCluster, this.ValidateCluster },
                    { (int)ClientPackets.validateLogin, this.ValidateLogin }
                };
            }
        }

        /// <summary>
        /// Cleanup the registered clusters.
        /// </summary>
        /// <param name="clientId">The client to disconnect.</param>
        internal override void DisconnectClient(int clientId)
        {
            base.DisconnectClient(clientId);

            clusterIds.Remove(clientId);
        }
    }
}
