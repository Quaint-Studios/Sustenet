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

namespace Sustenet.Transport.Messages.MasterHandlers
{
    using BaseServerHandlers;
    using Master;
    using Network;
    using System;
    using Utils.Security;

    /// <summary>
    /// Handles sending data to a client or cluster client from a server.
    /// </summary>
    static class MasterSend
    {
        #region Initialization Section
        /// <summary>
        /// Sends a Client their validated login information.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="toClient">The client to give a username and ID.</param>
        /// <param name="username">The client's username.</param>
        internal static void InitializeLogin(this MasterServer server, int toClient, string username)
        {
            MasterServer.DebugServer(server.serverTypeName, $"Setting Client#{toClient}'s username to {username}.");

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
        internal static void InitializeCluster(this MasterServer server, int toClient, string clusterName, string ip, ushort port)
        {
            server.clusterIds.Add(toClient); // Store the ID as a cluster since they've been verified.
            server.clusterInfo.Add(toClient, new World.ClusterInfo(clusterName, ip, port));

            using(Packet packet = new Packet((int)ServerPackets.initializeCluster))
            {
                packet.Write(clusterName);

                server.SendTcpData(toClient, packet);
            }
        }

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
            if(!RSAManager.KeyExists(keyName))
            {
                // ...do absolutely nothing. Just stay silent
                return;
            }

            // ...otherwise, serve a passphrase.

            string passphrase = PassphraseGenerator.GeneratePassphrase();
            AESManager.EncryptedData data = AESManager.Encrypt(keyName, passphrase);

            server.clients[toClient].name = passphrase; // Set the client name to the passphrase to store it.

            using(Packet packet = new Packet((int)ServerPackets.passphrase))
            {
                packet.Write(keyName);
                packet.Write(Convert.ToBase64String(data.cypher));
                packet.Write(Convert.ToBase64String(data.iv));

                server.SendTcpData(toClient, packet);
            }
        }
        #endregion

        #region Request Section
        /// <summary>
        /// TODO DOCUMENTATION
        /// </summary>
        /// <param name="server"></param>
        /// <param name="toClient"></param>
        internal static void SendClusterServers(this MasterServer server, int toClient)
        {
            MasterServer.DebugServer(server.serverTypeName, $"Sending Client#{toClient} a list of requested cluster servers.");

            using(Packet packet = new Packet((int)ServerPackets.clusterServerList))
            {
                packet.Write(server.clusterInfo.Count);

                foreach(World.ClusterInfo cluster in server.clusterInfo.Values)
                {
                    packet.Write(cluster.name);
                    packet.Write(cluster.ip);
                    packet.Write(cluster.port);
                }

                server.SendTcpData(toClient, packet);
            }

        }
        #endregion
    }
}
