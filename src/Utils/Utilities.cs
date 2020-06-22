using System;
using System.Collections.Generic;
using System.Text;
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
        #endregion
    }
}
