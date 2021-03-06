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

namespace Sustenet.Transport.Messages.BaseServerHandlers
{
    using Network;

    /// <summary>
    /// All the messages that the Base Server might send.
    /// </summary>
    static class BaseServerSend
    {
        /// <summary>
        /// Sends a basic message to a client.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="toClient">The client to send to.</param>
        /// <param name="msg">The message to send.</param>
        internal static void Message(this BaseServer server, int toClient, string msg)
        {
            using(Packet packet = new Packet((int)ServerPackets.message))
            {
                packet.Write(msg);

                server.SendTcpData(toClient, packet);
            }
        }

        #region Initialization Section
        /// <summary>
        /// Tells a client that a UDP connnection is ready.
        /// </summary>
        /// <param name="server">The server to run this on.</param>
        /// <param name="toClient">The client to notify that the connection is ready.</param>
        internal static void UdpReady(this BaseServer server, int toClient)
        {
            using(Packet packet = new Packet((int)ServerPackets.udpReady))
            {
                server.SendUdpData(toClient, packet);
            }
        }
        #endregion
    }
}
