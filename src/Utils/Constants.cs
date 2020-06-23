using System;
using System.Collections.Generic;
using System.Text;

namespace Sustenet.Utils
{
    class Constants
    {
        public const bool DEBUGGING = false;

        /// <summary>
        /// How many ticks are in a second.
        /// </summary>
        public const int TICK_RATE = 30;
        public const int MS_PER_TICK = 1000 / TICK_RATE;
    }
}
