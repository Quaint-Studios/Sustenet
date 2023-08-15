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

namespace Sustenet.Utils
{
    public class Constants
    {
        internal const bool DEBUGGING = true;

        /// <summary>
        /// How many ticks are in a second.
        /// </summary>
        internal const int TICK_RATE = 30;
        internal const int MS_PER_TICK = 1000 / TICK_RATE;

        public const ushort MASTER_PORT = 6256;
        public const ushort CLUSTER_PORT = 6257;
    }
}
