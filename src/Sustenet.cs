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

namespace Sustenet
{
    using System;
    using System.Collections.Generic;
    using NDesk.Options;
    using Utils;

    class Options
    {
        /// <summary>
        /// Loads command-line arguments. Defaults --master if no other connection type is provided.
        /// </summary>
        /// <param name="args"></param>
        /// <returns></returns>
        public static string[] GetOptions(string[] args)
        {
            List<string> connectionTypes = new List<string>();

            // TODO: Add functionality for duplicates of each connection type.
            OptionSet options = new OptionSet()
            {
                {
                    "client",
                    "starts a client and waits for Connect() to be triggered.",
                    v => { if(!connectionTypes.Contains("client")) connectionTypes.Add("client"); }
                },
                {
                    "cluster",
                    "starts a cluster server and uses the config file to connect to a master server.",
                    v => { if(!connectionTypes.Contains("cluster")) connectionTypes.Add("cluster"); }
                },
                {
                    "master",
                    "starts a master server, uses the config file to set it up, and waits for clusters and clients to connect.",
                    v => { if(!connectionTypes.Contains("master")) connectionTypes.Add("master"); }
                }
            };

            List<string> extra;
            try
            {
                extra = options.Parse(args);

                if(connectionTypes.Count <= 0)
                {
                    connectionTypes.Add("master");
                }

                // TODO: Do something with extra.

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
            Console.Title = "Sustenet";

            string[] options = Options.GetOptions(args);

            foreach(string option in options)
            {
                switch(option)
                {
                    case "client":
                        // Only to be used for debugging.
                        int maxClients = 5000; // TEST: Breaks on Windows after ~500 connections. More tests required.
                        Clients.Client[] clients = new Clients.Client[maxClients];
                        for(int i = 0; i < maxClients; i++)
                        {
                            clients[i] = new Clients.Client();
                        }
                        break;

                    case "cluster":
                        // TODO: var config = Utils.Config.GetConfig("ClusterServer");
                        World.ClusterServer cluster = new World.ClusterServer();
                        break;

                    case "master":
                        var masterConfig = Config.GetConfig(Config.ConfigType.MasterServer);

                        int? maxConnections = null;
                        if(masterConfig["maxConnections"] != null)
                        {
                            Utilities.TryParseNullable(masterConfig["maxConnections"].Value, out maxConnections);
                        }

                        ushort? port = null;
                        if(masterConfig["port"] != null)
                        {
                            Utilities.TryParseNullable(masterConfig["port"].Value, out port);
                        }

                        Master.MasterServer master = new Master.MasterServer(maxConnections ?? 0, port ?? 6256);
                        break;
                }
            }

            // Wait for the user to respond before closing.
            Console.WriteLine("Press any key to close Sustenet...");
            Console.ReadKey();
        }
    }
}
