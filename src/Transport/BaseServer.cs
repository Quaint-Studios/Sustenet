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
    using System.Collections.Generic;
    using System.Net;
    using System.Net.Sockets;
    using Utils;

    /// <summary>
    /// Base class of all server types.
    /// </summary>
    class BaseServer
    {
        public enum ServerType
        {
            MasterServer,
            ClusterServer
        }

        public bool isListening = false;

        private TcpListener tcpListener;

        public ServerType serverType;
        public int maxConnections;
        public ushort port;

        public Dictionary<int, BaseClient> clients = new Dictionary<int, BaseClient>();

        protected BaseServer(int _maxConnections = 0, ushort _port = 6256)
        {
            maxConnections = _maxConnections;
            port = _port == 0 ? (ushort)6256 : _port;

            Init();
        }

        /// <summary>
        /// Starts a server.
        /// </summary>
        /// <param name="serverType">The type of server to notify in the console.</param>
        protected void Start(ServerType _serverType)
        {
            serverType = _serverType;

            string serverTypeName = Utilities.SplitByPascalCase(serverType.ToString());

            Console.WriteLine($"===== Starting {serverTypeName} on Port {port} =====");

            tcpListener = new TcpListener(IPAddress.Any, port);
            tcpListener.Start();
            tcpListener.BeginAcceptTcpClient(new AsyncCallback(OnConnectCallback), this);

            string maxConnectionsValue = (maxConnections == 0 ? "Until it breaks" : Utilities.SplitByPascalCase(maxConnections.ToString()));

            Utilities.ConsoleHeader($"{serverTypeName} Started (Max connections: {maxConnectionsValue})");
        }

        /// <summary>
        /// Handles new connections.
        /// </summary>
        /// <param name="ar">Async Result, the state contains this instance.</param>
        private static void OnConnectCallback(IAsyncResult ar)
        {
            BaseServer server = (BaseServer)ar.AsyncState;
            TcpListener listener = server.tcpListener;

            TcpClient client = listener.EndAcceptTcpClient(ar);
            listener.BeginAcceptTcpClient(new AsyncCallback(OnConnectCallback), server);

            for(int id = 1; id <= server.maxConnections; id++)
            {
                if(server.clients[id].tcp.socket == null)
                {
                    server.clients[id].tcp.Connect(client);
                    return;
                }
            }

            Console.WriteLine($"{server.serverType.ToString()}: {client.Client.RemoteEndPoint} failed to connect. Max connections of {server.maxConnections} reached.");
        }

        protected void Init()
        {
            for(int id = 1; id <= maxConnections; id++)
            {
                clients.Add(id, new BaseClient(id));
            }
        }
    }
}
