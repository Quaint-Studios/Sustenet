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

namespace Sustenet.Transport.Messages.ClusterHandlers
{
    using Network;
    using Utils.Mathematics;
    using World;

    /// <summary>
    /// Any message the Cluster receives.
    /// </summary>
    static class ClusterReceive
    {
        #region Movement Section
        /// <summary>
        /// Verifies that a player has a legal velocity.
        /// </summary>
        /// <param name="server">The Cluster Server to run this on.</param>
        /// <param name="fromClient">The client sending this packet.</param>
        /// <param name="packet">The packet containing the 3 float positions.</param>
        internal static void ValidateMoveTo(this ClusterServer server, int fromClient, Packet packet)
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
            // You can check isGrounded by doing the following:
            // when Unity starts the server, setup a function that can be called and that returns true or false.
            // This way Unity can dictate this. The only concern is if it's thread safe.
            server.externalFuncs.IsGrounded.Invoke(fromClient);
            /// i.e. server.clientData[fromClient].pos do something

            // TODO: Update server-side velocity, send new position back and sync on the client-side.

            server.SendUpdatedPosition(fromClient, new float[] { 1.0005f, 2.512f, 3.245f });
        }
        #endregion
    }
}
