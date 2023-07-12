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

namespace Sustenet.Transport.Messages.ClusterClientHandlers
{
    using BaseClientHandlers;
    using Clients;
    using Network;
    using System.Net;
    using Utils;

    /// <summary>
    /// Any message that is outbound from the Cluster Client.
    ///
    /// ClusterClientSend/Receive [CCS/R]
    /// MasterSend/Receive [MS/R]
    /// CCS.ValidateCluster -> MR.ValidateCluster -> MS.Passphrase -> CCR.Passphrase -> CCS.AnswerPassphrase -> MR.AnswerPassphrase -> 
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
            IPHostEntry ipHostInfo = Dns.GetHostEntry(Dns.GetHostName()); // `Dns.Resolve()` method is deprecated.
            IPAddress ipAddress = ipHostInfo.AddressList[0];

            ushort? port = null;
            Utilities.TryParseNullable(Config.settings["port"].Value, out port);

            using(Packet packet = new Packet((int)ClientPackets.answerPassphrase))
            {
                packet.Write(answer);
                packet.Write(client.name);
                packet.Write(ipAddress.ToString());
                packet.Write(port ?? (ushort)6256);

                client.SendTcpData(packet);
            }
        }
    }
}
