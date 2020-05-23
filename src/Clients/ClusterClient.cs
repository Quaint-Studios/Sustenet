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

namespace Sustenet.Clients
{
    using Network;
    using Transport.Messages;

    class ClusterClient : Client
    {
        public ClusterClient(string _ip = "127.0.0.1", ushort _port = 6256, bool debug = true) : base(_ip, _port, debug)
        {
            tcp.onConnected.Run += () => this.ValidateCluster("ClusterTestName");

            // TODO: Load a private key from config.
        }

        protected override void InitializeClientData()
        {
            base.InitializeClientData();

            packetHandlers.Add((int)ServerPackets.initializeCluster, this.InitializeCluster);
            packetHandlers.Add((int)ServerPackets.passphrase, this.Passphrase);
        }
    }
}
