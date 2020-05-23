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
    using System;
    using Network;

    /// <summary>
    /// The core for the Base Client's message system.
    /// </summary>
    static class BaseClientCore
    {
        /// <summary>
        /// Sends a packet through the current stream.
        /// </summary>
        /// <param name="packet">The packet to be sent.</param>
        internal static void SendData(this BaseClient client, Packet packet)
        {
            try
            {
                if(client.tcp.socket == null)
                {
                    throw new Exception("TCPHandler socket is null.");
                }

                client.tcp.stream.BeginWrite(packet.ToArray(), 0, packet.Length(), null, null);
            }
            catch(Exception e)
            {
                client.tcp.onDebug.RaiseEvent($"Error sending data via TCP to Client#{client.id}...: {e}");
            }
        }
    }

    /// <summary>
    /// Messages that are sent from the Base Client.
    /// </summary>
    static class BaseClientSend
    {

    }

    /// <summary>
    /// Messages that are received by the Base Client.
    /// </summary>
    static class BaseClientReceive
    {

    }
}
