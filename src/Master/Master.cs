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

using System;

namespace Sustenet.Master
{
    using System.Collections.Generic;
    using System.Net;
    using TransportLayer;

    /// <summary>
    /// The Master Server keeps track of all Cluster Servers. It also allocates connecting users to Cluster Servers automatically, or allows the users to manually select one.
    /// </summary>
    public class Master
    {
        public IPAddress host = IPAddress.Loopback;
        public ushort port = 0;

        private readonly TransportLayer transport;

        public struct ClusterData
        {
            string _name;
            public string Name
            {
                get { return _name; }
                set { _name = value; }
            }

            IPAddress _host;
            public IPAddress Host
            {
                get { return _host; }
                set { _host = value; }
            }

            ushort _port;
            public ushort Port
            {
                get { return _port; }
                set { _port = value; }
            }

            uint _connections;
            public uint Connections
            {
                get { return _connections; }
                set { _connections = value; }
            }

            public ClusterData(string name, IPAddress host, ushort port)
            {
                _name = name;
                _host = host;
                _port = port;
                _connections = 0;
            }
        }

        public Dictionary<string, ClusterData> clusters;

        /// <summary>
        /// Creates a Transport Layer and prepares other functions.
        /// </summary>
        public Master()
        {
            if (port == 0)
            {
                port = 6256; // Default port for Sustenet.
            }

            transport = new TransportLayer(this);

            Init();
        }

        /// <summary>
        /// Initialize the Master server.
        /// </summary>
        void Init()
        {
            transport.Listen();

            while (transport.isListening)
            {
                // Jobs, send / receive, etc
            }
        }

        /// <summary>
        /// Adds only information required for indexing a cluster.
        /// </summary>
        void AddCluster()
        {
            ClusterData testCluster = new ClusterData();

            // TODO: If ClusterData.Name is a taken key in the clusters Dictionary, throw an error.
            clusters.Add(testCluster.Name, testCluster);
        }
    }
}
