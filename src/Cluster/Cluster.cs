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

namespace Sustenet.Cluster
{
    using System.Collections.Generic;
    using System.Net.Sockets;
    using TransportLayer;

    class Cluster : Server
    {
        public struct ClientData
        {
            string _name;
            public string Name
            {
                get { return _name; }
                set { _name = value; }
            }

            public ClientData(string name)
            {
                _name = name;
            }
        }

        public Dictionary<string, ClientData> clients = new Dictionary<string, ClientData>();

        /// <summary>
        /// Creates a Transport Layer and prepares other functions.
        /// </summary>
        public Cluster() : base()
        {

        }

        protected override void Init()
        {
            TransportLayerResponse responses = new TransportLayerResponse
            {
                OnListening = OnListening,

                OnConnect = OnConnect,
                OnDisconnect = OnDisconnect,

                OnMessageSent = OnMessageSent,
                OnMessageReceived = OnMessageReceived,

                OnShutdown = OnShutdown
            };

            transport.Listen(responses);
        }

        void OnListening(Socket handler)
        {

        }

        void OnConnect()
        {

        }

        void OnDisconnect()
        {

        }

        void OnMessageSent()
        {

        }

        void OnMessageReceived()
        {

        }

        void OnShutdown()
        {

        }
    }
}
