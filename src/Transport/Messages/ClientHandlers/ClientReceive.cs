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

namespace Sustenet.Transport.Messages.ClientHandlers
{
    using Network;
    using Clients;

    /// <summary>
    /// Messages that the Client would receive from a server.
    /// </summary>
    static class ClientReceive
    {
        /// <summary>
        /// Handle a message from the server.
        /// </summary>
        /// <param name="client">The client who received the message.</param>
        /// <param name="packet">The packet containing the message from the server.</param>
        internal static void Message(this Client client, Packet packet)
        {
            string msg = packet.ReadString();

            BaseClient.DebugClient(client.id, $"(Server Message) {msg}");
        }

        /// <summary>
        /// Initializes the client's ID and username.
        /// If the client is a Cluster, the username is the key.
        /// 
        /// TODO: Change to the cluster config name in the future.
        /// </summary>
        /// <param name="client">The client whose ID and username should be changed.</param>
        /// <param name="packet">The packet containing the new client ID.</param>
        internal static void InitializeClient(this Client client, Packet packet)
        {
            string username = packet.ReadString();
            int id = packet.ReadInt();

            client.name = username;
            client.id = id;

            BaseClient.DebugClient(client.id, $"Welcome, {username}!");
        }

        /// <summary>
        /// Updates the client's position. Some prediction should be included to smooth things out.
        /// </summary>
        /// <param name="client"></param>
        /// <param name="packet"></param>
        internal static void UpdatePosition(this Client client, Packet packet)
        {
            float xPos = packet.ReadFloat();
            float yPos = packet.ReadFloat();
            float zPos = packet.ReadFloat();
            BaseClient.DebugClient(client.id, $"Moved to ({xPos}, {yPos}, {zPos})");
        }
    }
}
