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
    using Network;
    using World;

    static class ClusterHandler
    {
        #region Command Functions
        /// <summary>
        /// Sends a packet to the master server that requests a string of text that must be
        /// decrypted and sent back.
        /// </summary>
        /// <param name="server">The cluster server requesting access.</param>
        /// <param name="keyName">The name of the SSH key stored on the master server. These are
        /// preloaded so there's no need to sanitize directory requests.</param>
        internal static void RegisterCluster(this ClusterServer server, string keyName)
        {
            using(Packet packet = new Packet((int)ClientPackets.cluster))
            {
                packet.Write(keyName);

                server.masterConn.SendData(packet);
            }
        }
        #endregion
    }
}
