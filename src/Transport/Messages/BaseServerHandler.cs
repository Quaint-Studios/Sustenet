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

    /// <summary>
    /// The core for the Base Server's message system.
    /// </summary>
    static class BaseServerCore
    {
        /// <summary>
        /// Send data to a single client.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="toClient">The client to send data to.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpData(this BaseServer server, int toClient, Packet packet)
        {
            packet.WriteLength();
            server.clients[toClient].SendData(packet);
        }

        /// <summary>
        /// Send data to all clients.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpDataToAll(this BaseServer server, Packet packet)
        {
            packet.WriteLength();
            foreach(BaseClient client in server.clients.Values)
            {
                client.SendData(packet);
            }
        }

        /// <summary>
        /// Send data to all clients except one.
        /// </summary>
        /// <param name="server">The server to send from.</param>
        /// <param name="exceptClient">The client to exclude from the mass send.</param>
        /// <param name="packet">The packet to send.</param>
        internal static void SendTcpDataToAll(this BaseServer server, int exceptClient, Packet packet)
        {
            packet.WriteLength();
            foreach(BaseClient client in server.clients.Values)
            {
                if(client.id != exceptClient)
                {
                    client.SendData(packet);
                }
            }
        }
    }

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
    }

    /// <summary>
    /// Messages that the Base Server receives from a Client.
    /// </summary>
    static class BaseServerReceive
    {

    }
}
