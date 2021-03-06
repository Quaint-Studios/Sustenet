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

namespace Sustenet
{
    using System;
    using System.Collections.Generic;
    using NDesk.Options;

    class Options
    {
        public class OptionsData
        {
            public bool client = false;
            public int maxClients = 1;
            public bool cluster = false;
            public bool master = false;
        }

        /// <summary>
        /// Loads command-line arguments. Defaults --master if no other connection type is provided.
        /// </summary>
        /// <param name="args"></param>
        /// <returns></returns>
        public static OptionsData GetOptions(string[] args)
        {
            OptionsData data = new OptionsData();

            // TODO: Add functionality for duplicates of each connection type.
            OptionSet options = new OptionSet()
            {
                {
                    "client:",
                    "starts a client and waits for Connect() to be triggered.",
                    v => {
                        if(v != null)
                        {
                            int.TryParse(v, out data.maxClients);
                        }

                        data.client = true;
                    }
                },
                {
                    "cluster",
                    "starts a cluster server and uses the config file to connect to a master server.",
                    v => { data.cluster = true; }
                },
                {
                    "master",
                    "starts a master server, uses the config file to set it up, and waits for clusters and clients to connect.",
                    v => { data.master = true; }
                }
            };

            List<string> extra;
            try
            {
                extra = options.Parse(args);

                // If nothing is set, set master to true.
                if(data.master == false && data.cluster == false && data.client == false)
                {
                    data.master = true;
                }

                return data;
            }
            catch(OptionException e)
            {
                Console.Write("Sustenet: ");
                Console.WriteLine(e.Message);
                Console.WriteLine("Try `sustenet --help' for more information.");
                throw;
            }
        }
    }
}
