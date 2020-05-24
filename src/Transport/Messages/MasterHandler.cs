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

namespace Sustenet.Transport.Messages
{
    using Master;
    using Network;
    using Utils;

    /// <summary>
    /// The core for all Master Server messages.
    /// </summary>
    static class MasterCore { }

    /// <summary>
    /// Handles sending data to a client or cluster client from a server.
    /// </summary>
    static class MasterSend
    {
        /// <summary>
        /// Generates a 128-156 character passphrase, encrypts it using an RSA key,
        /// stores the passphrase in the potential Cluster Client's name, and then
        /// sends it to the potential Cluster Client.
        /// </summary>
        /// <param name="server">The server to run this on.</param>
        /// <param name="toClient">The client to send the passphrase to.</param>
        /// <param name="keyName">The name of the key to use.</param>
        internal static void Passphrase(this MasterServer server, int toClient, string keyName)
        {
            // If the key doesn't exists...
            if(!Security.Keys.KeyExists(keyName))
            {
                // ...do absolutely nothing. Just stay silent
                return;
            }

            // ...otherwise, serve a passphrase.

            string passphrase = Security.GeneratePassphrase();
            string cypher = Security.Keys.Encrypt(keyName, passphrase);

            server.clients[toClient].name = cypher; // Set the client name to the cipher to store it.

            using(Packet packet = new Packet((int)ServerPackets.passphrase))
            {
                packet.Write(keyName);
                packet.Write(cypher);

                server.SendTcpData(toClient, packet);
            }
        }

        /// <summary>
        /// Sends a Client their validated login information.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="toClient">The client to give a username and ID.</param>
        /// <param name="username">The client's username.</param>
        internal static void InitializeLogin(this MasterServer server, int toClient, string username)
        {
            server.onDebug.RaiseEvent($"Setting Client#{toClient}'s username to {username}.");

            /**
             * TODO:
             * 1. There's no API decided currently. But, when the time comes, the user should authenticate through that.
             * 2. For now, just receive a username and let them use that name. No real validation needs to take place yet.
             * 3. Think about making it flexible enough to allow users to import their own auth systems.
             */
            using(Packet packet = new Packet((int)ServerPackets.initializeLogin))
            {
                packet.Write(username);
                packet.Write(toClient);

                server.SendTcpData(toClient, packet);
            }
        }

        /// <summary>
        /// Turns a Client into a Cluster.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="toClient">The client to send this to.</param>
        /// <param name="clusterName">The name the client requested and to send back to them.</param>
        internal static void InitializeCluster(this MasterServer server, int toClient, string clusterName)
        {
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

            server.clusterClients[id] = server.clients[toClient];

            server.clusterClients[id].tcp.onDisconnected.ClearEvents();
            server.clusterClients[id].tcp.onDisconnected.Run += () => server.ClearClusterClient(id);

            server.ClearClient(toClient);

            server.onConnection.RaiseEvent(id);

            //TODO: Send a packet back with the Cluster's keyName to let them know they're verified.
        }
    }

    /// <summary>
    /// Handles data the master server may receive.
    /// </summary>
    static class MasterReceive
    {
        /// <summary>
        /// Gives the client an ID and checks if the current username belongs to them.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="toClient">The client's new ID.</param>
        /// <param name="username">The client's username to validate.</param>
        internal static void ValidateLogin(this MasterServer server, int fromClient, Packet packet)
        {
            string username = packet.ReadString();

            // If the username's length is less than 3, disconnect the client and warn them.
            if(username.Length < 3)
            {
                server.Message(fromClient, "Please enter a username longer than 2 characters. Disconnecting.");
                server.DisconnectClient(fromClient);

                server.onDebug.RaiseEvent($"Disconnecting Client#{fromClient} for having the username '{username}' which is too short.");

                return;
            }
        }

        /// <summary>
        /// Starts the Cluster Client verification process.
        /// </summary>
        /// <param name="server"></param>
        /// <param name="fromClient"></param>
        /// <param name="packet"></param>
        internal static void ValidateCluster(this MasterServer server, int fromClient, Packet packet)
        {
            string keyName = packet.ReadString();

            server.Passphrase(fromClient, keyName);
        }

        /// <summary>
        /// Handles the client's answer. If it matches the client's name
        /// then initialize them as a Cluster Client.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="fromClient">The client sending this packet.</param>
        /// <param name="packet">The packet containing the answer and extra data.</param>
        internal static void AnswerPassphrase(this MasterServer server, int fromClient, Packet packet)
        {
            string answer = packet.ReadString();
            string clusterName = packet.ReadString(); // The requested name for this cluster.

            if(answer == server.clients[fromClient].name)
            {
                server.InitializeCluster(fromClient, clusterName);
            }
            else
            {
                server.Message(fromClient, "Incorrect passphrase. Disconnecting.");
                server.DisconnectClient(fromClient);

                server.onDebug.RaiseEvent($"Disconnecting Client#{fromClient} for answering their passphrase incorrectly.");

                return;
            }
        }
    }
}
