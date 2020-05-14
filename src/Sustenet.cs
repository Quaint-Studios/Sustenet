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
using System.Collections.Generic;
using NDesk.Options;

namespace Sustenet
{
    class Options
    {

        public static string[] GetOptions(string[] args)
        {
            List<string> connectionTypes = new List<string>();

            OptionSet options = new OptionSet()
            {
                {
                    "client",
                    "starts a client and waits for Connect() to be triggered.",
                    v => connectionTypes.Add("client")
                },
                {
                    "cluster",
                    "starts a cluster server and uses the config file to connect to a master server.",
                    v => connectionTypes.Add("cluster")
                },
                {
                    "master",
                    "starts a master server, uses the config file to set it up, and waits for clusters and clients to connect.",
                    v => connectionTypes.Add("master")
                }
            };

            List<string> extra;
            try
            {
                extra = options.Parse(args);
                Console.WriteLine(string.Join(",", extra));
                Console.WriteLine(string.Join(",", connectionTypes));
                return connectionTypes.ToArray();
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

    class Sustenet
    {
        static void Main(string[] args)
        {
            string[] options = Options.GetOptions(args);

            if(options.Length == 0)
            {
                var config = Utils.Config.GetConfig("MasterServer");
                Console.WriteLine("T:" + config["test"]);
                Console.WriteLine("F:" + string.Join(",", config));
                Master.MasterServer master = new Master.MasterServer();
            }
            else
            {
                bool handled = false;
                foreach(string option in options)
                {
                    switch(option)
                    {
                        case "client":
                            handled = true;
                            Clients.Client client = new Clients.Client();
                            break;

                        case "cluster":
                            handled = true;
                            // TODO: var config = Utils.Config.GetConfig("ClusterServer");
                            World.Cluster cluster = new World.Cluster();
                            break;

                        case "master":
                            handled = true;
                            var config = Utils.Config.GetConfig("MasterServer");
                            Console.WriteLine(string.Join(",", config));
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
