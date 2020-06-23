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
    using Utils;
    using Transport;
    using System.Threading;

    class Sustenet
    {
        private static bool isRunning = false;

        public static Clients.Client[] clients;
        public static World.ClusterServer cluster;
        public static Master.MasterServer master;

        static void Main(string[] args)
        {
            Console.Title = "Sustenet";

            Options.OptionsData options = Options.GetOptions(args);

            if(options.client)
            {
                // Only to be used for debugging.
                clients = new Clients.Client[options.maxClients];
                for(int i = 0; i < options.maxClients; i++)
                {
                    clients[i] = new Clients.Client();
                    clients[i].Connect();
                }
                Utilities.WriteLine($"Finished connecting {options.maxClients} clients to the server.");
            }

            if(options.cluster)
            {
                // TODO: var config = Utils.Config.GetConfig("ClusterServer");
                cluster = new World.ClusterServer();
            }

            if(options.master)
            {
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

                master = new Master.MasterServer(maxConnections ?? 0, port ?? 6256);
            }

            isRunning = true;
            Thread logicThread = new Thread(new ThreadStart(UpdateMain));
            logicThread.Name = "Logic Thread";
            logicThread.Start();

            // Wait for the user to respond before closing.
            Console.WriteLine("Press any key to close Sustenet...");
            Console.ReadKey();
            isRunning = false;
        }

        private static void UpdateMain()
        {
            DateTime next = DateTime.Now;

            while(isRunning)
            {
                while(next < DateTime.Now)
                {
                    ThreadManager.UpdateMain();

                    next = next.AddMilliseconds(Constants.MS_PER_TICK);

                    if(DateTime.Compare(next, DateTime.Now) > 0)
                    {
                        Thread.Sleep((int)(next - DateTime.Now).TotalMilliseconds);
                    }
                }
            }
        }
    }
}
