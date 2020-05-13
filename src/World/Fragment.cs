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

namespace Sustenet.World
{
    /// <summary>
    /// A single server that typically hosts a specific in-game location. It's
    /// used to split users up and to apply different settings based on the load
    /// for that in-game location.
    /// 
    /// The Fragment is ran on the same physical server as the Cluster but provides
    /// added flexibility with the ability to differentiate which users should receive
    /// certain update rates or which ones are in a certain location.
    /// </summary>
    class Fragment
    {
    }
}
