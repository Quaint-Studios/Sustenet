using System;
using System.Collections.Generic;
using System.Text;

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
    }
}
