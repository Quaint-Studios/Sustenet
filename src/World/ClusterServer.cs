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

namespace Sustenet.World
{
    using System.Timers;
    using Transport;
    using Transport.Messages;
    using Clients;

    /// <summary>
    /// A regionally hosted server that controls and allocates users to
    /// smaller fragmented servers.
    /// </summary>
    class ClusterServer : BaseServer
    {
        internal Client masterConn = new Client();
        private readonly Timer timer; // TODO: Only make this active if the server is command-line and not in Unity.

        /// <summary>
        /// Creates a Cluster Server that creates Fragment Servers to be used.
        /// TODO: Will currently only create a single server for itself.
        /// </summary>
        public ClusterServer(int _maxConnections = 0, ushort _port = 6257) : base(_maxConnections, _port)
        {
            timer = new Timer(20);
            // Hook up the Elapsed event for the timer.
            timer.Elapsed += UpdateMain;
            timer.AutoReset = true;
            timer.Enabled = true;

            Start(ServerType.ClusterServer);
            masterConn.tcp.onConnected.Run += () => this.RegisterCluster("ClusterTestName");
            masterConn.Connect();
        }

        public void UpdateMain(object source, ElapsedEventArgs e)
        {
            ThreadManager.UpdateMain();
        }
    }
}