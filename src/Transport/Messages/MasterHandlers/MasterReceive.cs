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

namespace Sustenet.Transport.Messages.MasterHandlers
{
    using BaseServerHandlers;
    using Master;
    using Network;
    using Utils.Mathematics;

    /// <summary>
    /// Handles data the master server may receive.
    /// </summary>
    static class MasterReceive
    {
        #region Initialization Section
        /// <summary>
        /// Gives the client an ID and checks if the current username belongs to them.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="toClient">The client's new ID.</param>
        /// <param name="username">The client's username to validate.</param>
        internal static void ValidateLogin(this MasterServer server, int fromClient, Packet packet)
        {
            string username = packet.ReadString();

            // If the username's length is less than 3, disconnect the client and warn them.
            if(username.Length < 3)
            {
                server.Message(fromClient, "Please enter a username longer than 2 characters. Disconnecting.");
                server.DisconnectClient(fromClient);

                MasterServer.DebugServer(server.serverTypeName, $"Disconnecting Client#{fromClient} for having the username '{username}' which is too short.");

                return;
            }

            server.InitializeLogin(fromClient, username);
        }

        /// <summary>
        /// Starts the Cluster Client verification process.
        /// </summary>
        /// <param name="server"></param>
        /// <param name="fromClient"></param>
        /// <param name="packet"></param>
        internal static void ValidateCluster(this MasterServer server, int fromClient, Packet packet)
        {
            string keyName = packet.ReadString();

            server.Passphrase(fromClient, keyName);
        }

        /// <summary>
        /// Handles the client's answer. If it matches the client's name
        /// then initialize them as a Cluster Client.
        /// </summary>
        /// <param name="server">The Master Server to run this on.</param>
        /// <param name="fromClient">The client sending this packet.</param>
        /// <param name="packet">The packet containing the answer and extra data.</param>
        internal static void AnswerPassphrase(this MasterServer server, int fromClient, Packet packet)
        {
            string answer = packet.ReadString();
            string clusterName = packet.ReadString(); // The requested name for this cluster.
            string ip = packet.ReadString();
            ushort port = packet.ReadUShort();

            if(answer == server.clients[fromClient].name)
            {
                server.InitializeCluster(fromClient, clusterName, ip, port);
            }
            else
            {
                server.Message(fromClient, "Incorrect passphrase. Disconnecting.");
                server.DisconnectClient(fromClient);

                MasterServer.DebugServer(server.serverTypeName, $"Disconnecting Client#{fromClient} for answering their passphrase incorrectly.");

                return;
            }
        }
        #endregion

        #region Request Section
        /// <summary>
        /// TODO DOCUMENTATION
        /// </summary>
        /// <param name="server"></param>
        /// <param name="fromClient"></param>
        /// <param name="packet"></param>
        internal static void RequestClusterServers(this MasterServer server, int fromClient, Packet packet)
        {
            server.SendClusterServers(fromClient);
        }
        #endregion

        #region Movement Section
        /// <summary>
        /// Verifies that a player has a legal velocity.
        /// </summary>
        /// <param name="server">The Cluster Server to run this on.</param>
        /// <param name="fromClient">The client sending this packet.</param>
        /// <param name="packet">The packet containing the 3 float positions.</param>
        internal static void ValidateMoveTo(this MasterServer server, int fromClient, Packet packet)
        {
            float xPos = packet.ReadFloat();
            float yPos = packet.ReadFloat();
            float zPos = packet.ReadFloat();

            // Validations
            if(!xPos.InRange(-5, 5) || !yPos.InRange(-5, 5) || !zPos.InRange(-5, 5))
            {
                return;
            }

            // TODO: PSEUDO => if zPos != 0 && !grounded, cancel the operation.

            // TODO: Update server-side velocity, send new position back and sync on the client-side.

            server.SendUpdatedPosition(fromClient, new float[] { 1.0005f, 2.512f, 3.245f });
        }
        #endregion
    }
}
