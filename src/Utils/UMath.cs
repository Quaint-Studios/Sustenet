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

namespace Sustenet.Utils.Mathematics
{
    /// <summary>
    /// The Math Utility class.
    /// </summary>
    public static class UMath
    {
        #region InRange
        // TODO: Expand on this, adding more overloads and types.

        /// <summary>
        /// Determines if a number is in range of min and max (inclusive).
        /// </summary>
        public static bool InRange(this int num, int min, int max)
        {
            return num >= min && num <= max;
        }

        /// <summary>
        /// Determines if a number is in range of min and max (inclusive).
        /// </summary>
        public static bool InRange(this float num, float min, float max)
        {
            return num >= min && num <= max;
        }
        #endregion
    }
}
