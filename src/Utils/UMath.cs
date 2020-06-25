using System;
using System.Collections.Generic;
using System.Text;

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
