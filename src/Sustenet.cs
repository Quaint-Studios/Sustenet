﻿/**
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

namespace Sustenet
{
    class Sustenet
    {
        static void Main(string[] args)
        {
            if(args.Length == 0)
            {
                Master.MasterServer master = new Master.MasterServer();
            }
            else
            {
                bool handled = false;
                foreach(string arg in args)
                {
                    switch(arg)
                    {
                        case "client":
                            handled = true;
                            Clients.Client client = new Clients.Client();
                            break;

                        case "cluster":
                            handled = true;
                            World.Cluster cluster = new World.Cluster();
                            break;

                        case "master":
                            handled = true;
                            Master.MasterServer master = new Master.MasterServer();
                            break;
                    }

                    // If an argument has been handled.
                    if(handled)
                        break;
                }
            }

            // Wait for the user to respond before closing.
            Console.Write("Press any key to close Sustenet...");
            Console.ReadKey();
        }
    }
}
