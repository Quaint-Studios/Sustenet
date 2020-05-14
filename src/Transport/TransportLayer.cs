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
    using System.Net.Sockets;
    using Master;

    public struct TransportLayerResponse
    {
        // TODO: Convert to events delegates.
        public Action<Socket> OnListening;

        public Action OnConnect;
        public Action OnDisconnect;

        public Action OnMessageSent;
        public Action OnMessageReceived;

        public Action OnShutdown;
    }

    /// <summary>
    /// TODO
    /// </summary>
    static class TransportLayer
    {
        public static void Listen(TransportLayerResponse responses, BaseServer server)
        {
            server.isListening = true;

            Console.WriteLine($"Listening on port {server.port} (TCP/UDP).");
            // listen for incoming traffic here.

            // TODO: Return an event for the server to use with updates.
        }
    }
}
