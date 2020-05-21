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
    using Clients;

    /// <summary>
    /// Handles functionality for clients.
    /// </summary>
    static class ClientHandler
    {
        #region Receive Command Functions
        /// <summary>
        /// Handle a message from the server.
        /// </summary>
        /// <param name="client">The client who received the message.</param>
        /// <param name="packet">The packet containing the message from the server.</param>
        internal static void Message(this Client client, Packet packet)
        {
            string msg = packet.ReadString();

            client.tcp.onDebug.RaiseEvent($"(Server Message) {msg}");
        }

            int id = packet.ReadInt();

            client.id = id;

            client.tcp.onDebug.RaiseEvent($"(Server says)...: {msg}");
        }
        #endregion

        #region Send Command Functions
        /// <summary>
        /// Sends a request to the server to login.
        /// TODO: Authentication and persistent sessions.
        /// </summary>
        /// <param name="client">The client requesting to login.</param>
        /// <param name="username">The username to login as.</param>
        internal static void Login(this Client client, string username)
        {
            if(client.activeConnection == Client.ConnectionType.MasterServer)
            {
                using(Packet packet = new Packet((int)ClientPackets.login))
                {
                    packet.Write(username);

                    client.SendData(packet);
                }
            }
            else
            {
                client.tcp.onDebug.RaiseEvent("Cannot login unless connected to a Master Server.");
            }
        }
        #endregion
    }
}
