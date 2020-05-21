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
    using Master;
    using Network;

    /// <summary>
    /// Handles sending data to a client from a server.
    /// </summary>
    static class MasterHandler
    {
        #region Command Functions
        internal static void ValidateCluster(this MasterServer server, int fromClient, Packet packet)
        {
            /**
             * TODO:
             * 1. Load all public keys in ./cfg/keys.
             * 2. Store the keys in memory using a Dictionary<string (the filename), string (the content)>().
             * 3. When a cluster requests access, encrypt a random string of text that varies in size and wait 5 seconds.
             * 4. If 5 seconds passes and a response isn't given, disconnect the cluster.
             * 5. If a cluster gives the wrong response, disconnect it.
             * 6. If a specific IP gives the wrong response a predefined (Defined in ./cfg/MasterServer.config) amount of
             *    time, add it to a list of banned IPs. 0 will result in never banning. 1 bans on the first mistake.
             * 7. If answered correctly, move the client's info to the cluster Dictionary and send a ServerPackets.clusterWelcome
             */
            string keyName = packet.ReadString();

            int id;

            if(server.releasedClusterIds.Count > 0)
            {
                id = server.releasedClusterIds[0];
                server.clusterClients.Add(id, null); // Reserve this spot instantly.

                server.releasedClusterIds.RemoveAt(0);
            }
            else
            {
                id = server.clusterClients.Count;
                server.clusterClients.Add(id, null); // Reserve this spot instantly here too.
            }

            server.clusterClients[id] = server.clients[fromClient];

            server.clusterClients[id].tcp.onDisconnected.ClearEvents();
            server.clusterClients[id].tcp.onDisconnected.Run += () => server.ClearClusterClient(id);

            server.ClearClient(fromClient);

            server.onConnection.RaiseEvent(id);
        }

        /// <summary>
        /// Gives the client an ID and checks if the current username belongs to them.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="toClient">The client's new ID.</param>
        /// <param name="username">The client's username to validate.</param>
        internal static void ValidateUser(this MasterServer server, int fromClient, Packet packet)
        {
            string username = packet.ReadString();

            // If the username's length is less than 3, disconnect the client and warn them.
            if(username.Length < 3)
            {
                using(Packet packetResponse = new Packet((int)ServerPackets.message))
                {
                    packetResponse.Write("Please enter a username longer than 2 characters. Disconnecting.");

                    server.SendTcpData(fromClient, packet);
                    server.DisconnectClient(fromClient);
                }
                server.onDebug.RaiseEvent($"Disconnecting Client#{fromClient} for having the username \"{username}\" which is too short.");

                return;
            }

            server.onDebug.RaiseEvent($"Setting Client#{fromClient}'s username to {username}.");

            /**
             * TODO:
             * 1. There's no API decided currently. But, when the time comes, the user should authenticate through that.
             * 2. For now, just receive a username and let them use that name. No real validation needs to take place yet.
             * 3. Think about making it flexible enough to allow users to import their own auth systems.
             */
            using(Packet packetResponse = new Packet((int)ServerPackets.validateUser))
            {
                packetResponse.Write(username);
                packetResponse.Write(fromClient);

                server.SendTcpData(fromClient, packet);
            }
        }
        #endregion
    }
}
