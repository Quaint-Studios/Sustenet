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
    using System.Collections.Generic;
    using System.Net;
    using System.Net.Sockets;
    using Transport;

    /// <summary>
    /// The Master Server keeps track of all Cluster Servers. It also allocates connecting users to Cluster Servers automatically, or allows the users to manually select one.
    /// </summary>
    class MasterServer : BaseServer
    {
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

        public Dictionary<string, ClusterData> clusters = new Dictionary<string, ClusterData>();

        /// <summary>
        /// Creates a Transport Layer and prepares other functions.
        /// </summary>
        public MasterServer() : base()
        {

        }

        /// <summary>
        /// Initialize the Master server.
        /// </summary>
        protected override void Init()
        {
            #region Test (Cluster example data)
            ClusterData[] testClusters = new ClusterData[]{
                new ClusterData("World 0", IPAddress.Parse("10.8.0.2"), 6575) { Connections = 9024 },
                new ClusterData("World 1", IPAddress.Parse("10.8.0.3"), 6576) { Connections = 240 }
            };

            foreach(ClusterData cluster in testClusters)
            {
                AddCluster(cluster);
            }
            #endregion

            TransportLayerResponse responses = new TransportLayerResponse
            {
                OnListening = OnListening,

                OnConnect = OnConnect,
                OnDisconnect = OnDisconnect,

                OnMessageSent = OnMessageSent,
                OnMessageReceived = OnMessageReceived,

                OnShutdown = OnShutdown
            };

            TransportLayer.Listen(responses, this); // TODO: Should return events. Subscribe to them.
        }

        void OnListening(Socket handler)
        {

        }

        void OnConnect()
        {

        }

        void OnDisconnect()
        {

        }

        void OnMessageSent()
        {

        }

        void OnMessageReceived()
        {

        }

        void OnShutdown()
        {

        }

        /// <summary>
        /// Adds only information required for indexing a cluster.
        /// </summary>
        /// <returns>The success status of adding the cluster. Fails if key exists.</returns>
        bool AddCluster(ClusterData cluster)
        {
            if(clusters.ContainsKey(cluster.Name))
            {
                return false;
            }

            // TODO: If ClusterData.Name is a taken key in the clusters Dictionary, throw an error.
            clusters.Add(cluster.Name, cluster);

            return true;
        }
    }
}
