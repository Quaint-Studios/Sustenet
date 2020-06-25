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

namespace Sustenet.Transport.Messages.BaseServerHandlers
{
    using System;
    using Network;
    using BaseClientHandlers;

    /// <summary>
    /// The core for the Base Server's message system.
    /// </summary>
    static class BaseServerCore
    {
        /// <summary>
        /// Send TCP data to a single client.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="toClient">The client to send data to.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpData(this BaseServer server, int toClient, Packet packet)
        {
            try
            {
                server.clients[toClient].SendTcpData(packet);
            }
            catch(Exception e)
            {
                BaseClient.DebugClient(toClient, $"Error sending data via TCP from Client #{toClient}...: {e}");
            }
        }

        /// <summary>
        /// Send UDP data to a single client.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="toClient">The client to send data to.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendUdpData(this BaseServer server, int toClient, Packet packet)
        {
            try
            {
                BaseClient client = server.clients[toClient];

                if(client.udp.endPoint != null)
                {
                    BaseClient.UdpHandler.socket.BeginSend(packet.ToArray(), packet.Length(), client.udp.endPoint, null, null);
                }
            }
            catch(Exception e)
            {
                BaseClient.DebugClient(toClient, $"Error sending data via UDP from Client #{toClient}...: {e}");
            }
        }

        /// <summary>
        /// Send TCP data to all clients.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpDataToAll(this BaseServer server, Packet packet)
        {
            foreach(BaseClient client in server.clients.Values)
            {
                client.SendTcpData(packet);
            }
        }

        /// <summary>
        /// Send UDP data to all clients.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendUdpDataToAll(this BaseServer server, Packet packet)
        {
            foreach(BaseClient client in server.clients.Values)
            {
                client.SendTcpData(packet);
            }
        }

        /// <summary>
        /// Send TCP data to all clients except one.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="exceptClient">The client to exclude from the mass send.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpDataToAll(this BaseServer server, int exceptClient, Packet packet)
        {
            foreach(BaseClient client in server.clients.Values)
            {
                if(client.id != exceptClient)
                {
                    client.SendUdpData(packet);
                }
            }
        }

        /// <summary>
        /// Send UDP data to all clients except one.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="exceptClient">The client to exclude from the mass send.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendUdpDataToAll(this BaseServer server, int exceptClient, Packet packet)
        {
            foreach(BaseClient client in server.clients.Values)
            {
                if(client.id != exceptClient)
                {
                    client.SendUdpData(packet);
                }
            }
        }
    }
}