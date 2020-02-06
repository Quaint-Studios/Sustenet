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
    using System.Net;
    using System.Threading;
    using System.Text;
    using System.Collections.Generic;

    class TCPSocket
    {
        #region Server
        public class Server
        {
            private readonly string address;
            private readonly ushort port;

            private Socket server;
            public List<Socket> clients = new List<Socket>();

            TransportLayerResponse responses;

            public Server(string address = "0.0.0.0", ushort port = 6256, TransportLayerResponse responses = default)
            {
                Console.WriteLine("==== Starting Server ====");

                this.address = address;
                this.port = port;

                this.responses = responses;

                StartServer();
            }

            // State object for reading client data asynchronously  
            public class StateObject
            {
                // Client  socket.  
                public Socket workSocket = null;
                // Size of receive buffer.  
                public const int BufferSize = 1024;
                // Receive buffer.  
                public byte[] buffer = new byte[BufferSize];
                // Received data string.  
                public StringBuilder sb = new StringBuilder();
            }

            private void StartServer()
            {
                // Bind the socket to the local endpoint and listen for incoming connections.  
                try
                {
                    Listen();
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            private void Listen()
            {
                if(!IPAddress.TryParse(address, out IPAddress ipAddress))
                {
                    // Establish the local endpoint for the socket.
                    IPHostEntry ipHostInfo = Dns.GetHostEntry(Dns.GetHostName());
                    ipAddress = ipHostInfo.AddressList[0];
                }

                IPEndPoint localEndPoint = new IPEndPoint(ipAddress, port);

                // Create a TCP/IP socket.  
                server = new Socket(ipAddress.AddressFamily, SocketType.Stream, ProtocolType.Tcp);

                server.Bind(localEndPoint);
                server.Listen(100);

                // Start an asynchronous socket to listen for connections.  
                Console.WriteLine("[Server]: Waiting for a connection...");
                server.BeginAccept(new AsyncCallback(OnServerListening), server);
            }

            private void OnServerListening(IAsyncResult ar)
            {
                // Get the socket that handles the client request.  
                Socket listener = (Socket)ar.AsyncState;
                Socket handler = listener.EndAccept(ar);

                clients.Add(handler);
                responses.OnListening?.Invoke(handler); // Add support for responses.

                // Create the state object.  
                StateObject state = new StateObject
                {
                    workSocket = handler
                };
                handler.BeginReceive(state.buffer, 0, StateObject.BufferSize, 0, new AsyncCallback(OnMessageReceived), state);
                listener.BeginAccept(new AsyncCallback(OnServerListening), listener);
            }

            private static void OnMessageReceived(IAsyncResult ar)
            {
                String content = String.Empty;

                // Retrieve the state object and the server socket  
                // from the asynchronous state object.  
                StateObject state = (StateObject)ar.AsyncState;
                Socket handler = state.workSocket;

                // Read data from the client socket.   
                int bytesRead = handler.EndReceive(ar);

                if(bytesRead > 0)
                {
                    // There  might be more data, so store the data received so far.  
                    state.sb.Append(Encoding.ASCII.GetString(state.buffer, 0, bytesRead));

                    // Check for end-of-file tag. If it is not there, read   
                    // more data.  
                    content = state.sb.ToString();
                    if(content.IndexOf("<EOF>") > -1)
                    {
                        // All the data has been read from the   
                        // client. Display it on the console.  
                        Console.WriteLine("[Server]: \"{1}\" ({0} bytes)", content.Length, content);

                        // Echo the data back to the client.  
                        Echo(handler, content);
                    }
                    else
                    {
                        // Not all data received. Get more.  
                        handler.BeginReceive(state.buffer, 0, StateObject.BufferSize, 0, new AsyncCallback(OnMessageReceived), state);
                    }
                }
            }

            public static void Echo(Socket handler, String data)
            {
                // Convert the string data to byte data using ASCII encoding.  
                byte[] byteData = Encoding.ASCII.GetBytes(data);

                // Begin sending the data to the remote device.  
                handler.BeginSend(byteData, 0, byteData.Length, 0, new AsyncCallback(OnMessageSent), handler);
            }


            public void Send(String data)
            {
                // Convert the string data to byte data using ASCII encoding.  
                byte[] byteData = Encoding.ASCII.GetBytes(data);

                // Begin sending the data to the remote device.  
                clients[0].BeginSend(byteData, 0, byteData.Length, 0, new AsyncCallback(OnMessageSent), clients[0]);
            }

            private static void OnMessageSent(IAsyncResult ar)
            {
                try
                {
                    // Retrieve the socket from the state object.  
                    Socket handler = (Socket)ar.AsyncState;

                    // Complete sending the data to the remote device.  
                    int bytesSent = handler.EndSend(ar);
                    Console.WriteLine("[Server]: Sent message to client. ({0} bytes)", bytesSent);
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            public void Shutdown()
            {
                TCPSocket.Shutdown(server);
            }
        }
        #endregion

        #region Client
        public class Client
        {
            private static String response = String.Empty;

            private readonly string address;
            private readonly ushort port;

            private Socket client;

            public Client(string address, ushort port)
            {
                Console.WriteLine("==== Starting Client ====");

                this.address = address;
                this.port = port;

                StartClient();
            }

            // State object for receiving data from remote device.  
            public class StateObject
            {
                // Client socket.  
                public Socket workSocket = null;
                // Size of receive buffer.  
                public const int BufferSize = 256;
                // Receive buffer.  
                public byte[] buffer = new byte[BufferSize];
                // Received data string.  
                public StringBuilder sb = new StringBuilder();
            }

            private void StartClient()
            {
                try
                {
                    Connect();
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            private void Connect()
            {
                if(!IPAddress.TryParse(address, out IPAddress ipAddress))
                {
                    IPHostEntry ipHostInfo = Dns.GetHostEntry(address);
                    ipAddress = ipHostInfo.AddressList[0];
                }

                IPEndPoint remoteEP = new IPEndPoint(ipAddress, port);

                Console.WriteLine("[Client]: Connecting to {0}...", remoteEP.ToString());

                client = new Socket(ipAddress.AddressFamily, SocketType.Stream, ProtocolType.Tcp);

                client.BeginConnect(remoteEP, new AsyncCallback(OnClientConnected), client);
            }

            private void OnClientConnected(IAsyncResult ar)
            {
                try
                {
                    // Retrieve the socket from the state object.  
                    Socket client = (Socket)ar.AsyncState;

                    // Complete the connection.
                    client.EndConnect(ar);

                    Console.WriteLine("[Client]: Connected to {0}.", client.RemoteEndPoint.ToString());

                    Receive();
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            public void Send(String data)
            {
                // Convert the string data to byte data using ASCII encoding.  
                byte[] byteData = Encoding.ASCII.GetBytes(data);

                // Begin sending the data to the remote device.  
                client.BeginSend(byteData, 0, byteData.Length, 0, new AsyncCallback(OnMessageSent), client);
            }
            private static void OnMessageSent(IAsyncResult ar)
            {
                try
                {
                    // Retrieve the socket from the state object.  
                    Socket client = (Socket)ar.AsyncState;

                    // Complete sending the data to the remote device.  
                    int bytesSent = client.EndSend(ar);
                    Console.WriteLine("[Client]: Sent message to server. ({0} bytes)", bytesSent);
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            private void Receive()
            {
                try
                {
                    // Create the state object.  
                    StateObject state = new StateObject
                    {
                        workSocket = client
                    };

                    // Begin receiving the data from the remote device.  
                    client.BeginReceive(state.buffer, 0, StateObject.BufferSize, 0, new AsyncCallback(OnMessageReceived), state);
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            private static void OnMessageReceived(IAsyncResult ar)
            {
                try
                {
                    // Retrieve the state object and the client socket   
                    // from the asynchronous state object.  
                    StateObject state = (StateObject)ar.AsyncState;
                    Socket client = state.workSocket;

                    // Read data from the remote device.  
                    int bytesRead = client.EndReceive(ar);

                    if(bytesRead > 0)
                    {
                        // There might be more data, so store the data received so far.  
                        state.sb.Append(Encoding.ASCII.GetString(state.buffer, 0, bytesRead));

                        // Get the rest of the data.  
                        client.BeginReceive(state.buffer, 0, StateObject.BufferSize, 0, new AsyncCallback(OnMessageReceived), state);
                    }
                    else
                    {
                        // All the data has arrived; put it in response.  
                        if(state.sb.Length > 1)
                        {
                            response = state.sb.ToString();
                        }

                        Console.WriteLine("[Client]: \"{0}\" ({1} bytes)", response, response.Length);
                    }
                }
                catch(Exception e)
                {
                    Console.WriteLine(e.ToString());
                }
            }

            public void Shutdown()
            {
                // Release the socket.  
                TCPSocket.Shutdown(client);
            }
        }
        #endregion

        private static void Shutdown(Socket socket)
        {
            // Release the socket.  
            socket.Shutdown(SocketShutdown.Both);
            socket.Close();
        }
    }
}
