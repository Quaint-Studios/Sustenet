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
using System.Text;

namespace Sustenet.Clients
{
    using System.Net;
    using Transport;

    /// <summary>
    /// A standard client that connects to a server.
    /// </summary>
    class Client : BaseClient
    {
        public IPAddress ip;
        public ushort port = 6256;


        public Client(string _ip = "127.0.0.1", ushort _port = 6256) : base(0)
        {
            ip = IPAddress.Parse(_ip);

            tcp.Connect(ip, port);
        }
    }
}
