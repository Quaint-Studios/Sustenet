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
    using BaseClientHandlers;
    using Utils;
    using System;

    /// <summary>
    /// TODO: Documentation
    /// </summary>
    static class ClientSend
    {
        #region Initialization Section
        /// <summary>
        /// Sends a request to the server to login.
        /// TODO: Authentication and persistent sessions.
        /// </summary>
        /// <param name="client">The client requesting to login.</param>
        /// <param name="username">The username to login as.</param>
        internal static void ValidateLogin(this Client client, string username)
        {
            if(client.activeConnection == Client.ConnectionType.MasterServer)
            {
                using(Packet packet = new Packet((int)ClientPackets.validateLogin))
                {
                    packet.Write(username);

                    client.SendTcpData(packet);
                }
            }
            else
            {
                BaseClient.DebugClient(client.id, "Cannot login unless connected to a Master Server.");
            }
        }

        /// <summary>
        /// Sends a packet with only the client ID to start a UDP connection.
        /// </summary>
        /// <param name="client">The client requesting for a UDP connection.</param>
        internal static void StartUdp(this Client client)
        {
            // Tell the server to create an endpoint with this client
            using(Packet packet = new Packet((int)ClientPackets.startUdp))
            {
                client.SendUdpData(packet);
            }
        }
        #endregion

        #region Movement Section
        /// <summary>
        /// Moves the client in the desired direction.
        /// </summary>
        /// <param name="client">The client to move.</param>
        /// <param name="velocity">How fast they should move (validated by the server).</param>
        internal static void MoveTo(this Client client, float[] velocity)
        {
            if(velocity == null || velocity.Length < 3)
            {
                BaseClient.DebugClient(client.id, "The velocity is either null or does not have an x, y, z.");
                return;
            }

            // PENDING UPDATE: Change to ConnectionType.ClusterServer once the ClusterServer is actually being used.
            if(client.activeConnection == Client.ConnectionType.MasterServer)
            {
                using(Packet packet = new Packet((int)ClientPackets.moveTo))
                {
                    packet.Write(velocity[0]);
                    packet.Write(velocity[1]);
                    packet.Write(velocity[2]);

                    client.SendUdpData(packet);
                }
            }
            else
            {
                BaseClient.DebugClient(client.id, "Can't move until connected to a Cluster Server.");
            }
        }
        #endregion
    }
}
