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
    using World;

    /// <summary>
    /// The core all Cluster messages.
    /// </summary>
    static class ClusterCore { }

    /// <summary>
    /// Any message that is outbound from the Cluster or goes through the cluster to
    /// get to its client connection to the Master Server.
    /// </summary>
    static class ClusterSend
    {
        /// <summary>
        /// Gives the client an ID and asks the Master Server if the current username belongs to them.
        /// </summary>
        /// <param name="server">The Cluster Server to run this on.</param>
        /// <param name="toClient">The client's new ID.</param>
        /// <param name="username">The client's username to validate.</param>
        internal static void InitializeLogin(this ClusterServer server, int toClient, string username)
        {
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
        /// Sends a packet to the master server that requests a string of text that must be
        /// decrypted and sent back.
        /// </summary>
        /// <param name="server">The cluster server requesting access.</param>
        /// <param name="keyName">The name of the SSH key stored on the master server. These are
        /// preloaded so there's no need to sanitize directory requests.</param>
        internal static void ValidateCluster(this ClusterServer server, string keyName)
        {
            using(Packet packet = new Packet((int)ClientPackets.validateCluster))
            {
                packet.Write(keyName);

                server.masterConn.SendData(packet);
            }
        }

        /// <summary>
        /// Placeholder for answering the passphrase the server may send to the cluster.
        /// </summary>
        internal static void AnswerCluster() { }
    }

    /// <summary>
    /// Any message the Cluster receives.
    /// </summary>
    static class ClusterReceive
    {

    }
}
