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
        /// Gives the client an ID.
        /// </summary>
        welcome = 1,
        /// <summary>
        /// Turns a regular client into a Cluster Client and gives it a
        /// new ID. Should only be used from a Master Server.
        /// </summary>
        clusterWelcome = 2
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
        /// Request to be a Cluster. If authenticated, the client
        /// will get a new ID and will be moved to a server
        /// dictionary on the Master server.
        /// </summary>
        cluster = -1,
        /// <summary>
        /// Logs in with a username.
        /// </summary>
        login = 1
    }
}
