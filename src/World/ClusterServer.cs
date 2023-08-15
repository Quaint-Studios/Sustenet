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

namespace Sustenet.World
{
    using Clients;
    using Network;
    using System;
    using System.Collections.Generic;
    using System.Numerics;
    using Transport;
    using Transport.Messages.ClusterHandlers;
    using Utils;

    /// <summary>
    /// TODO RE-EVALULATE
    /// TODO DOCUMENTATION
    /// </summary>
    public struct ClusterInfo
    {
        public string name;
        public string ip;
        public ushort port;

        public ClusterInfo(string _name, string _ip, ushort _port)
        {
            name = _name;
            ip = _ip;
            port = _port;
        }
    }

    /// <summary>
    /// A regionally hosted server that controls and allocates users to
    /// smaller fragmented servers.
    /// </summary>
    public class ClusterServer : BaseServer
    {
        /// <summary>
        /// Data used to validate a client's actions.
        /// </summary>
        internal class ClientData
        {
            internal Vector3 pos;

            public ClientData(Vector3 _pos)
            {
                pos = _pos;
            }
        }

        public struct ExternalFuncs
        {
            public Predicate<int> IsGrounded;

            public ExternalFuncs(Predicate<int> _IsGrounded)
            {
                IsGrounded = _IsGrounded;
            }
        }

        public ExternalFuncs externalFuncs;
        public ClusterClient masterConn = new ClusterClient(Config.settings["masterIp"].Value, ushort.TryParse(Config.settings["port"].Value, out ushort port) ? port : Constants.MASTER_PORT);
        internal Dictionary<int, ClientData> clientData = new Dictionary<int, ClientData>();

        /// <summary>
        /// Creates a Cluster Server that creates Fragment Servers to be used.
        /// TODO: Will currently only create a single server for itself.
        /// </summary>
        public ClusterServer(int _maxConnections = 0, ushort _port = Constants.CLUSTER_PORT) : base(ServerType.ClusterServer, _maxConnections, _port)
        {
            InitializeData();

            Start(ServerType.ClusterServer);

            // When a user connects, load their data in.
            // Todo: needs to get actual data from the master server.
            // Also needs to pass that data back occasionally.
            onConnection.Run += (id) => clientData.Add(id, new ClientData(_pos: new Vector3(0, 0, 0)));

            masterConn.Connect();
        }

        /// <summary>
        /// Creates a Cluster Server that creates Fragment Servers to be used.
        /// For Unity this will allow passing external functions.
        /// TODO: Will currently only create a single server for itself.
        /// </summary>
        public ClusterServer(ExternalFuncs _externalFuncs, int _maxConnections = 0, ushort _port = Constants.CLUSTER_PORT) : base(ServerType.ClusterServer, _maxConnections, _port)
        {
            externalFuncs = _externalFuncs;

            InitializeData();

            Start(ServerType.ClusterServer);

            masterConn.Connect();
        }

        protected virtual void InitializeData()
        {
            if(packetHandlers == null)
            {
                packetHandlers = new Dictionary<int, PacketHandler>()
                {
                    #region Movement Section
                    { (int)ClientPackets.moveTo, this.ValidateMoveTo }
                    #endregion
                };
            }
        }
    }
}
