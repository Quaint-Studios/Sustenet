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
    using Transport.Core.Spawning;
    using World;

    /// <summary>
    /// The core of all Cluster messages.
    /// </summary>
    static class ClusterCore
    {
        public static void SpawnPlayer(this ClusterServer cluster, int toClient, Player player)
        {
            BaseClient client = cluster.clients[toClient];
            client.player = player;

            // TODO: Split the cluster's clients into fragments so updates are only sent to those within a certain fragmented group and at a reduced rate to neighboring groups.
            foreach(BaseClient otherClient in cluster.clients.Values)
            {
                if(otherClient.player != null)
                {
                    if(otherClient.id != client.id)
                    {
                        // Spawn
                    }
                }
            }

            foreach(BaseClient otherClient in cluster.clients.Values)
            {
                if(otherClient.player != null)
                {
                    // Spawn
                }
            }
        }
    }
}
