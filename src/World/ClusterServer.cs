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
    using System.Collections.Generic;
    using Transport;
    using Utils;

    internal struct ClusterInfo
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
        public ClusterClient masterConn = new ClusterClient(Config.settings["masterIp"].Value, ushort.TryParse(Config.settings["port"].Value, out ushort port) ? port : (ushort)6256);

        /// <summary>
        /// Creates a Cluster Server that creates Fragment Servers to be used.
        /// TODO: Will currently only create a single server for itself.
        /// </summary>
        public ClusterServer(int _maxConnections = 0, ushort _port = 6257) : base(ServerType.ClusterServer, _maxConnections, _port)
        {
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

                };
            }
        }
    }
}
