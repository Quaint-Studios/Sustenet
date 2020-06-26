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
    using System.Threading.Tasks;
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
        // UDP equivalent is in BaseClient.UdpHandler.socket

        public readonly ServerType serverType;
        public readonly string serverTypeName;
        public readonly int maxConnections;
        public readonly ushort port;

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

        protected BaseServer(ServerType _serverType, int _maxConnections = 0, ushort _port = 6256)
        {
            serverType = _serverType;
            serverTypeName = Utilities.SplitByPascalCase(serverType.ToString());
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
            if(Constants.DEBUGGING)
            {
#pragma warning disable CS0162 // Unreachable code detected
                onConnection.Run += (id) => DebugServer(serverTypeName, $"Client#{id} has connected.");
#pragma warning restore CS0162 // Unreachable code detected
            }

            Utilities.ConsoleHeader($"Starting {serverTypeName} on Port {port}");

            tcpListener = new TcpListener(IPAddress.Any, port);
            tcpListener.Start();
            tcpListener.BeginAcceptTcpClient(new AsyncCallback(OnTcpConnectCallback), this);

            if(BaseClient.UdpHandler.socket == null)
            {
                BaseClient.UdpHandler.socket = new UdpClient(port);
            }

            BaseClient.UdpHandler.socket.BeginReceive(OnUdpReceiveCallback, this);

            string maxConnectionsValue = (maxConnections == 0 ? "Until it breaks" : Utilities.SplitByPascalCase(maxConnections.ToString()));

            Utilities.ConsoleHeader($"{serverTypeName} Started (Max connections: {maxConnectionsValue})");
        }

        /// <summary>
        /// Handles new connections.
        /// </summary>
        /// <param name="ar">Async Result, the state contains this instance.</param>
        private static void OnTcpConnectCallback(IAsyncResult ar)
        {
            BaseServer server = (BaseServer)ar.AsyncState;

            try
            {
                TcpListener listener = server.tcpListener;

                TcpClient client = listener.EndAcceptTcpClient(ar);
                listener.BeginAcceptTcpClient(new AsyncCallback(OnTcpConnectCallback), server);

                ThreadManager.ExecuteOnMainThread(() =>
                {
                    server.AddClient(client);
                });
            }
            catch(Exception e)
            {
                DebugServer(server.serverTypeName, $"Failed to create a client: {e}");
            }
        }

        private static void OnUdpReceiveCallback(IAsyncResult ar)
        {
            BaseServer server = (BaseServer)ar.AsyncState;

            try
            {
                IPEndPoint endpoint = new IPEndPoint(IPAddress.Any, 0);
                byte[] data = BaseClient.UdpHandler.socket.EndReceive(ar, ref endpoint);
                BaseClient.UdpHandler.socket.BeginReceive(OnUdpReceiveCallback, server);

                if(data.Length <= 4)
                {
                    // no ID
                    return;
                }

                using(Packet packet = new Packet(data))
                {
                    int clientId = packet.ReadInt();

                    if(!server.clients.ContainsKey(clientId))
                    {
                        return;
                    }

                    // If this is a new client, set their endpoint and return.
                    if(server.clients[clientId].udp.endpoint == null)
                    {
                        server.clients[clientId].udp.endpoint = endpoint;
                        return;
                    }

                    // Validate that the endpoints match to verify that the user is who they say they are.
                    if(server.clients[clientId].udp.endpoint.ToString() == endpoint.ToString())
                    {
                        server.clients[clientId].onReceived.RaiseEvent(Protocols.UDP, null);
                        server.HandleUdpData(server.clients[clientId].id, packet);
                    }
                }
            }
            catch(Exception e)
            {
                Utilities.WriteLine(e);
            }
        }

        private void AddClient(TcpClient client)
        {
            int id = -1;
            try
            {
                if(maxConnections == 0 || clients.Count < maxConnections)
                {
                    lock(clients)
                        lock(releasedIds)
                        {
                            // Loop until an ID is found.
                            while(id == -1)
                            {
                                // If there are released IDs, use one.
                                if(releasedIds.Count > 0)
                                {
                                    id = releasedIds[0];
                                    if(!clients.ContainsKey(id))
                                    {
                                        clients.Add(id, new BaseClient(id)); // Reserve this spot.
                                        releasedIds.Remove(id);
                                    }
                                    else
                                    {
                                        id = -1;
                                        continue;
                                    }
                                }
                                else
                                {
                                    // Assign the next highest client ID if there's no released IDs.
                                    id = clients.Count;

                                    if(!clients.ContainsKey(id))
                                    {
                                        clients.Add(id, new BaseClient(id)); // Reserve this spot here too.
                                    }
                                    else
                                    {
                                        id = -1;
                                        continue;
                                    }
                                }
                            }
                        }

                    clients[id].receivedData = new Packet();

                    clients[id].onReceived.Run += (protocol, data) =>
                    {
                        // Convert the BaseClient to work for the server.
                        switch(protocol)
                        {
                            case Protocols.TCP:
                                clients[id].receivedData.Reset(HandleTcpData(clients[id], data));
                                return;

                            case Protocols.UDP:
                                // Extra things to do goes here.
                                return;
                        }
                    };
                    clients[id].onConnected.Run += () =>
                    {
                        this.UdpReady(id);
                    };
                    // Clear the entry from the server.
                    clients[id].onDisconnected.Run += () => { ClearClient(id); };

                    clients[id].tcp.Receive(clients[id], client);

                    onConnection.RaiseEvent(id);

                    return;
                }

                DebugServer(serverTypeName, $"{client.Client.RemoteEndPoint} failed to connect. Max connections of {maxConnections} reached.");
            }
            catch
            {
                // If the id was never reset.
                // That means that a client may still exist.
                // Cleanup.
                if(id != -1)
                {
                    ClearClient(id);
                }
            }
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
        internal async void ClearClient(int clientId)
        {
            // TODO: Check if this increases performance at all.
            await Task.Run(() =>
            {
                try
                {
                    clients[clientId].Dispose();
                    clients.Remove(clientId);
                    releasedIds.Add(clientId); // TODO: Change this to only keep Ids of a certain reusable range.

                    DebugServer(serverTypeName, $"Disconnected Client#{clientId}.");
                }
                catch(Exception e)
                {
                    lock(clients)
                    {
                        if(clients.ContainsKey(clientId))
                        {
                            if(clients[clientId] != null)
                            {
                                clients[clientId].Dispose();
                            }

                            clients.Remove(clientId);
                            releasedIds.Add(clientId);
                        }
                        DebugServer(serverTypeName, $"Disconnected Client#{clientId} but with issues: {e}");
                    }
                }
            });
        }
        #endregion

        #region Data Functions
        private bool HandleTcpData(BaseClient client, byte[] data)
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

                ThreadManager.ExecuteOnMainThread(() =>
                {
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

        private bool HandleUdpData(int clientId, Packet packet)
        {
            int packetLength = packet.ReadInt();
            byte[] packetBytes = packet.ReadBytes(packetLength);

            ThreadManager.ExecuteOnMainThread(() =>
            {
                using(Packet packet = new Packet(packetBytes))
                {
                    int packetId = packet.ReadInt();
                    packetHandlers[packetId](clientId, packet);
                }
            });

            return false;
        }
        #endregion

        public static void DebugServer(string serverTypeName, string msg)
        {
            Utilities.WriteLine($"({serverTypeName}) {msg}");
        }
    }
}
