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

namespace Sustenet.Network
{
    /// <summary>
    /// What a server sends. What a client receives.
    /// </summary>
    public enum ServerPackets
    {
        /// <summary>
        /// Sends a passphrase that a Cluster Client should decrypt and answer.
        /// </summary>
        passphrase = -2,
        /// <summary>
        /// Turns a regular client into a Cluster Client and gives it a
        /// new ID. Should only be used from a Master Server.
        /// </summary>
        initializeCluster = -1,
        /// <summary>
        /// Send a standard message to the client.
        /// </summary>
        message = 0,
        /// <summary>
        /// Gives the client an ID. Validates the user locally from Master Server.
        /// If ran on a Cluster Server, asks the Master Server if they're actual
        /// a valid user.
        /// </summary>
        initializeLogin = 1,
        /// <summary>
        /// Tells a client that the UDP connection is ready.
        /// </summary>
        udpReady = 2,

        #region Movement (2000-2999)
        updatePosition = 2000 // TODO: Move to Cluster once clusters are being used.
        #endregion
    }

    /// <summary>
    /// What a client sends and what only a cluster should receive.
    /// </summary>
    public enum ClusterPackets
    {
        /// <summary>
        /// Sends movement updates.
        /// </summary>
        move = 1
    }

    /// <summary>
    /// What a client sends. What a server receives.
    /// </summary>
    public enum ClientPackets
    {
        /// <summary>
        /// Sends an answer to a passphrase to the server.
        /// </summary>
        answerPassphrase = -2,
        /// <summary>
        /// Request to be a Cluster. If authenticated, the client
        /// will get a new ID and will be moved to a server
        /// dictionary on the Master server.
        /// </summary>
        validateCluster = -1,
        /// <summary>
        /// Logs in with a username.
        /// </summary>
        validateLogin = 1,
        /// <summary>
        /// Sends an ID to a server to start a UDP connection.
        /// </summary>
        startUdp = 2,

        #region Movement (2000,2999)
        moveTo = 2000
        #endregion
    }
}
