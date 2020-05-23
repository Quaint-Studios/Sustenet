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

namespace Sustenet.Transport.Messages
{
    using Network;

    /// <summary>
    /// The core for the Base Server's message system.
    /// </summary>
    static class BaseServerCore
    {
        #region Data Functions
        internal static void SendTcpData(this BaseServer server, int toClient, Packet packet)
        {
            packet.WriteLength();
            server.clients[toClient].SendData(packet);
        }

        internal static void SendTcpDataToAll(this BaseServer server, Packet packet)
        {
            packet.WriteLength();
            foreach(BaseClient client in server.clients.Values)
            {
                client.SendData(packet);
            }
        }

        internal static void SendTcpDataToAll(this BaseServer server, int exceptClient, Packet packet)
        {
            packet.WriteLength();
            foreach(BaseClient client in server.clients.Values)
            {
                if(client.id != exceptClient)
                {
                    client.SendData(packet);
                }
            }
        }
        #endregion
    }

    /// <summary>
    /// All the messages that the Base Server might send.
    /// </summary>
    static class BaseServerSend
    {
        #region Command Functions
        internal static void Message(this BaseServer server, int toClient, string msg)
        {
            using(Packet packet = new Packet((int)ServerPackets.message))
            {
                packet.Write(msg);

                server.SendTcpData(toClient, packet);
            }
        }
        #endregion
    }

    /// <summary>
    /// Messages that the Base Server receives from a client.
    /// </summary>
    static class BaseServerReceive
    {

    }
}
