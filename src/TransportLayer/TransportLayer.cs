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

namespace Sustenet.TransportLayer
{
    using System;
    using System.Net.Sockets;

    public struct TransportLayerResponse
    {
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
    class TransportLayer
    {
        public bool isListening = false;

        public TCPSocket.Server tcpServer;
        public TCPSocket.Client tcpClient;

        public readonly UDPSocket udpSocket;
        public readonly UDPSocket udpSocket2;

        private readonly Server server;

        /// <summary>
        /// Initializes a Transport Layer.
        /// </summary>
        /// <param name="server">The master server that has all of the server data.</param>
        public TransportLayer(Server server)
        {
            this.server = server;
        }

        //
        public void Listen(TransportLayerResponse responses)
        {
            isListening = true;

            tcpServer = new TCPSocket.Server(port: server.port, responses: responses);

            Console.WriteLine($"Listening on port {server.port} (TCP/UDP).");
            // listen for incoming traffic here.
        }
    }
}
