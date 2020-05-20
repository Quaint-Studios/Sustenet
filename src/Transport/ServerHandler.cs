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
    using Network;

    static class ServerHandler
    {
        #region Command Functions

        // Packet ID = 1
        public static void Welcome(this BaseServer server, int toClient, string msg)
        {
            using(Packet packet = new Packet((int)ServerPackets.welcome))
            {
                packet.Write(msg);
                packet.Write(toClient);

                server.SendTcpData(toClient, packet);
            }
        }
        #endregion

        #region Data Functions
        private static void SendTcpData(this BaseServer server, int toClient, Packet packet)
        {
            packet.WriteLength();
            server.clients[toClient].tcp.SendData(packet);
        }

        private static void SendTcpDataToAll(this BaseServer server, Packet packet)
        {
            packet.WriteLength();
            foreach(BaseClient client in server.clients.Values)
            {
                client.tcp.SendData(packet);
            }
        }

        private static void SendTcpDataToAll(this BaseServer server, int exceptClient, Packet packet)
        {
            packet.WriteLength();
            foreach(BaseClient client in server.clients.Values)
            {
                if(client.id != exceptClient)
                {
                    client.tcp.SendData(packet);
                }
            }
        }
        #endregion
    }
}
