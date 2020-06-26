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

namespace Sustenet.Transport.Messages.BaseClientHandlers
{
    using System;
    using Network;

    /// <summary>
    /// The core for the Base Client's message system.
    /// </summary>
    static class BaseClientCore
    {
        /// <summary>
        /// Send data to the server.
        /// </summary>
        /// <param name="client">The client to run this on.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpData(this BaseClient client, Packet packet)
        {
            try
            {
                packet.WriteLength();
                if(client.tcp.socket == null)
                {
                    throw new Exception("TCPHandler socket is null.");
                }

                client.tcp.stream.BeginWrite(packet.ToArray(), 0, packet.Length(), null, null);
            }
            catch(Exception e)
            {
                BaseClient.DebugClient(client.id, $"Error sending data via TCP to Client#{client.id}...: {e}");
            }
        }

        internal static void SendUdpData(this BaseClient client, Packet packet)
        {
            try
            {
                if(client.id <= -1)
                {
                    BaseClient.DebugClient(client.id, "This client hasn't finished being setup by the server.");
                    return;
                }

                packet.WriteLength();
                packet.InsertInt(client.id);
                if(BaseClient.UdpHandler.socket != null)
                {
                    BaseClient.UdpHandler.socket.BeginSend(packet.ToArray(), packet.Length(), null, null);
                }
            }
            catch(Exception e)
            {
                BaseClient.DebugClient(client.id, $"Error sending data via UDP to Client #{client.id}...: {e}");
            }
        }
    }
}
