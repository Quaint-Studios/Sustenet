﻿/**
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
    using Events;
    using Network;
    using Utils;

    /// <summary>
    /// Base class of all server types. Takes in clients.
    /// </summary>
    class BaseServer
    {
        public enum ServerType
        {
            MasterServer,
            ClusterServer
        }

        private TcpListener tcpListener;

        public ServerType serverType;
        public int maxConnections;
        public ushort port;

        public Dictionary<int, BaseClient> clients = new Dictionary<int, BaseClient>();
        public List<int> releasedIds = new List<int>();

        protected delegate void PacketHandler(int fromClient, Packet packet);

        /// <summary>
        /// A dictionary on how packets should be handled.
        /// </summary>
        protected static Dictionary<int, PacketHandler> packetHandlers;

        public BaseEvent<int> onConnection = new BaseEvent<int>();
        public BaseEvent<int> onDisconnection = new BaseEvent<int>();
        public BaseEvent<byte[]> onReceived = new BaseEvent<byte[]>();
        public BaseEvent<string> onDebug = new BaseEvent<string>();

        protected BaseServer(int _maxConnections = 0, ushort _port = 6256)
        {
            maxConnections = _maxConnections;
            port = _port == 0 ? (ushort)6256 : _port;
        }

        #region Connection Functions
        /// <summary>
        /// Starts a server.
        /// </summary>
        /// <param name="serverType">The type of server to notify in the console.</param>
        protected void Start(ServerType _serverType)
        {
            serverType = _serverType;

            string serverTypeName = Utilities.SplitByPascalCase(serverType.ToString());

            onConnection.Run += (id) => DebugServer(serverTypeName, $"Client#{id} has connected.");
            onDebug.Run += (msg) => DebugServer(serverTypeName, msg);

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

                server.clients[id].receivedData = new Packet();

                server.clients[id].tcp.onReceived.Run += (data) => {
                    // Convert the BaseClient to work for the server.
                    server.clients[id].receivedData.Reset(server.HandleData(server.clients[id], data));
                };

                server.clients[id].tcp.Receive(client);

                server.onConnection.RaiseEvent(id);

                server.clients[id].tcp.onDisconnected.Run += () => server.ClearClient(id);

                return;
            }

            server.onDebug.RaiseEvent($"{client.Client.RemoteEndPoint} failed to connect. Max connections of {server.maxConnections} reached.");
        }

        /// <summary>
        /// Kicks a client off the server and clears their entry.
        /// </summary>
        /// <param name="clientId">The client id to kick and clear.</param>
        internal virtual void DisconnectClient(int clientId)
        {
            ClearClient(clientId);
        }

        /// <summary>
        /// Frees up a client ID by wiping them from the server list.
        /// </summary>
        /// <param name="clientId">The client id to free up.</param>
        internal void ClearClient(int clientId)
        {
            clients[clientId].Dispose();
            clients.Remove(clientId);
            releasedIds.Add(clientId);
            onDebug.RaiseEvent($"Disconnected Client#{clientId}.");
        }
        #endregion

        #region Data Functions
        private bool HandleData(BaseClient client, byte[] data)
        {

            int packetLength = 0;

            client.receivedData.SetBytes(data);

            if(client.receivedData.UnreadLength() >= 4)
            {
                packetLength = client.receivedData.ReadInt();
                if(packetLength <= 0)
                {
                    return true;
                }
            }

            while(packetLength > 0 && packetLength <= client.receivedData.UnreadLength())
            {
                byte[] packetBytes = client.receivedData.ReadBytes(packetLength);

                ThreadManager.ExecuteOnMainThread(() => {
                    using(Packet packet = new Packet(packetBytes))
                    {
                        int packetId = packet.ReadInt();
                        packetHandlers[packetId](client.id, packet);
                    }
                });

                packetLength = 0;

                if(client.receivedData.UnreadLength() >= 4)
                {
                    packetLength = client.receivedData.ReadInt();
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
        #endregion

        private static void DebugServer(string serverTypeName, string msg)
        {
            Console.WriteLine($"({serverTypeName}) {msg}");
        }
    }
}
