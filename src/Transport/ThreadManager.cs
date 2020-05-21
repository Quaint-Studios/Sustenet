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

namespace Sustenet.Transport
{
    using System;
    using System.Collections.Generic;

    //
    public sealed class ThreadManager
    {
        public static readonly ThreadManager instance = new ThreadManager();
        private readonly List<Action> mainPool, mainPoolCopied;
        private bool executeAction = false;

        public ThreadManager()
        {
            mainPool = new List<Action>();
            mainPoolCopied = new List<Action>();
        }

        /// <summary>
        /// Sets an action to be executed on the main thread.
        /// </summary>
        /// <param name="action">The action to be executed on the main thread.</param>
        public static void ExecuteOnMainThread(Action action)
        {
            if(action == null)
            {
                Console.WriteLine("No action to execute on main thread!");
                return;
            }

            lock(instance.mainPool)
            {
                instance.mainPool.Add(action);
                instance.executeAction = true;
            }
        }

        /// <summary>
        /// Execute all code meant to run on the main thread. Should only be called from the main thread.
        /// </summary>
        public static void UpdateMain()
        {
            if(instance.executeAction)
            {
                instance.mainPoolCopied.Clear();
                lock(instance.mainPool)
                {
                    instance.mainPoolCopied.AddRange(instance.mainPool);
                    instance.mainPool.Clear();
                    instance.executeAction = false;
                }

                instance.mainPoolCopied.ForEach((action) => action());
            }
        }
    }
}
