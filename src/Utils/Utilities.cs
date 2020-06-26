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

using System;
using System.Text.RegularExpressions;

namespace Sustenet.Utils
{
    public static class Utilities
    {
        #region TryParseNullable
        public static bool TryParseNullable(string n, out ushort? outVal)
        {
            ushort val;
            if(ushort.TryParse(n, out val))
            {
                outVal = val;
                return true;
            }
            else
            {
                outVal = null;
                return false;
            }
        }
        public static bool TryParseNullable(string n, out int? outVal)
        {
            int val;
            if(int.TryParse(n, out val))
            {
                outVal = val;
                return true;
            }
            else
            {
                outVal = null;
                return false;
            }
        }
        #endregion

        #region String Formatting
        /// <summary>
        /// Splits a text with spaces by pascal casing.
        /// </summary>
        /// <param name="t">The text to split.</param>
        /// <returns>The text, split with spaces.</returns>
        public static string SplitByPascalCase(string t)
        {
            // Regex to split by Pascal Casing and ignores the first character, which is a space.
            return Regex.Replace(t, "([A-Z]+(?=$|[A-Z][a-z])|[A-Z]?[a-z]+)", " $1").Substring(1);
        }

        /// <summary>
        /// Adds a header wrapper inside a Console.WriteLine(). It's just for reusability.
        /// </summary>
        /// <param name="h">The text to output.</param>
        public static void ConsoleHeader(string h)
        {
            Console.WriteLine($"===== {h} =====");
        }
        #endregion

        #region Files Handling
        public static string GetAppPath()
        {
            return System.IO.Path.GetDirectoryName(System.Reflection.Assembly.GetEntryAssembly().Location);
        }
        #endregion

        #region Debugging
        public static void WriteLine(string msg)
        {
            if(Constants.DEBUGGING)
            {
#pragma warning disable CS0162 // Unreachable code detected
                Console.WriteLine(msg);
#pragma warning restore CS0162 // Unreachable code detected
            }
        }

        public static void WriteLine(Exception e)
        {
            if(Constants.DEBUGGING)
            {
#pragma warning disable CS0162 // Unreachable code detected
                Console.WriteLine(e);
#pragma warning restore CS0162 // Unreachable code detected
            }
        }
        #endregion
    }
}
