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
    using Network;
    using Clients;

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
        /// <param name="client">The cluster server requesting access.</param>
        /// <param name="keyName">The name of the SSH key stored on the master server. These are
        /// preloaded so there's no need to sanitize directory requests.</param>
        internal static void ValidateCluster(this ClusterClient client, string keyName)
        {
            using(Packet packet = new Packet((int)ClientPackets.validateCluster))
            {
                packet.Write(keyName);

                client.SendData(packet);
            }
        }

        /// <summary>
        /// Placeholder for answering the passphrase the server may send to the cluster.
        /// </summary>
        internal static void AnswerPassphrase(this ClusterClient client, string answer)
        {
            using(Packet packet = new Packet((int)ClientPackets.answerPassphrase))
            {

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
        internal static void InitializeCluster(this Client client, Packet packet)
        {
            string keyName = packet.ReadString();


            client.tcp.onDebug.RaiseEvent($"Welcome, {keyName}!");
        }

        internal static void Passphrase(this Client client, Packet packet)
        {
            string passphrase = packet.ReadString();
        }
    }
}
