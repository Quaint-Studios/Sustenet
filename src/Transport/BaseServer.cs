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
        public List<int> releasedIds = new List<int>();

        protected BaseServer(int _maxConnections = 0, ushort _port = 6256)
        {
            maxConnections = _maxConnections;
            port = _port == 0 ? (ushort)6256 : _port;
        }

        /// <summary>
        /// Starts a server.
        /// </summary>
        /// <param name="serverType">The type of server to notify in the console.</param>
        protected void Start(ServerType _serverType)
        {
            serverType = _serverType;

            string serverTypeName = Utilities.SplitByPascalCase(serverType.ToString());

            Utilities.ConsoleHeader($"Starting {serverTypeName} on Port {port}");

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

            if(server.maxConnections == 0 || server.clients.Count < server.maxConnections)
            {
                int id;

                if(server.releasedIds.Count > 0)
                {
                    id = server.releasedIds[0];
                    server.clients.Add(id, null); // Reserve this spot instantly.

                    server.releasedIds.RemoveAt(0);
                }
                else
                {
                    id = server.clients.Count;
                    server.clients.Add(id, null); // Reserve this spot instantly here too.
                }

                server.clients[id] = new BaseClient(id);

                Console.WriteLine(id);

                server.clients[id].tcp.Receive(client);

                DebugServer(server.serverType, $"Client#{id} connected.");

                server.Welcome(id, $"Welcome to the {Utilities.SplitByPascalCase(server.serverType.ToString())}, Client#{id}.");

                return;
            }

            DebugServer(server.serverType, $"{client.Client.RemoteEndPoint} failed to connect. Max connections of {server.maxConnections} reached.");
        }

        /// <summary>
        /// Initializes the server's data.
        /// </summary>
        protected bool Init(int id)
        {
            try
            {
                clients.Add(id, new BaseClient(id));
                DebugServer(serverType, $"Client#{id} initialized.");

                return true;
            }
            catch
            {
                return false;
            }
        }

        private static void DebugServer(ServerType serverType, string msg)
        {
            Console.WriteLine($"{serverType.ToString()}: {msg}");
        }
    }
}
