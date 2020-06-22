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
    using Clients;
    using Network;
    using System;
    using Utils.Security;

    /// <summary>
    /// The core all Cluster Client messages.
    /// </summary>
    static class ClusterClientCore { }

    /// <summary>
    /// Any message that is outbound from the Cluster Client.
    /// </summary>
    static class ClusterClientSend
    {
        /// <summary>
        /// Sends a packet to the master server that requests a string of text that must be
        /// decrypted and sent back.
        /// </summary>
        /// <param name="client">The Cluster Client to run this from.</param>
        /// <param name="keyName">The name of the SSH key stored on the master server. These are
        /// preloaded so there's no need to sanitize directory requests.</param>
        internal static void ValidateCluster(this ClusterClient client, string keyName)
        {
            using(Packet packet = new Packet((int)ClientPackets.validateCluster))
            {
                packet.Write(keyName);

                client.SendTcpData(packet);
            }
        }

        /// <summary>
        /// Placeholder for answering the passphrase the server may send to the cluster.
        /// </summary>
        /// <param name="client">The cluster client</param>
        /// <param name="answer"></param>
        internal static void AnswerPassphrase(this ClusterClient client, string answer)
        {
            using(Packet packet = new Packet((int)ClientPackets.answerPassphrase))
            {
                packet.Write(answer);
                packet.Write(client.name);

                client.SendTcpData(packet);
            }
        }
    }

    /// <summary>
    /// Any message the Cluster Client receives.
    /// </summary>
    static class ClusterClientReceive
    {
        /// <summary>
        /// Initializes the client's ID and username.
        /// If the client is a Cluster, the username is the key.
        /// 
        /// TODO: Change to the cluster config name in the future.
        /// </summary>
        /// <param name="client">The client whose ID and username should be changed.</param>
        /// <param name="packet">The packet containing the new client ID.</param>
        internal static void InitializeCluster(this ClusterClient client, Packet packet)
        {
            string keyName = packet.ReadString();

            client.name = keyName;

            client.onDebug.RaiseEvent($"Welcome, {keyName}!");
        }

        /// <summary>
        /// Reads a keyName and passphrase from the server and attempts to answer it.
        /// </summary>
        /// <param name="client"></param>
        /// <param name="packet"></param>
        internal static void Passphrase(this ClusterClient client, Packet packet)
        {
            string keyName = packet.ReadString();
            byte[] cypher = Convert.FromBase64String(packet.ReadString());
            byte[] iv = Convert.FromBase64String(packet.ReadString());


            client.AnswerPassphrase(AESManager.Decrypt(keyName, cypher, iv));
        }
    }
}
