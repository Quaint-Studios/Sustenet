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

/** 
 * Thanks to Tom Weiland for providing the basis of this code which may be improved
 * over time.
 */

namespace Sustenet.Network
{
    using System;
    using System.Collections.Generic;
    using System.Text;

    class Packet : IDisposable
    {
        private List<byte> buffer = new List<byte>();
        private byte[] readableBuffer;
        private int readPos = 0;

        /// <summary>
        /// Creates an empty packet without an ID.
        /// </summary>
        public Packet() { }

        /// <summary>
        /// Creates an empty packet with an ID. Used for sending data.
        /// </summary>
        /// <param name="_id">The packet ID.</param>
        public Packet(int _id) { Write(_id); }

        /// <summary>
        /// Creates a packet and sets data to prepare it for reading. Used for receiving data.
        /// </summary>
        /// <param name="data">The bytes to add to the packet.</param>
        public Packet(byte[] data) { SetBytes(data); }

        #region Packet Functions
        /// <summary>
        /// Sets the packet's content and prepares it to be read.
        /// </summary>
        /// <param name="data">The bytes to add to the packet.</param>
        public void SetBytes(byte[] data)
        {
            Write(data);
            readableBuffer = buffer.ToArray();
        }

        /// <summary>
        /// Insert length of the packet's content at the start of the buffer.
        /// </summary>
        public void WriteLength()
        {
            buffer.InsertRange(0, BitConverter.GetBytes(buffer.Count));
        }

        /// <summary>
        /// Inserts an integer at the start of the buffer.
        /// </summary>
        /// <param name="data">The number to insert.</param>
        public void InsertInt(int data)
        {
            buffer.InsertRange(0, BitConverter.GetBytes(data));
        }

        /// <summary>
        /// Returns the packet's data as an array.
        /// </summary>
        public byte[] ToArray()
        {
            readableBuffer = buffer.ToArray();
            return readableBuffer;
        }

        /// <summary>
        /// The length of the packet's content.
        /// </summary>
        public int Length()
        {
            return buffer.Count;
        }

        /// <summary>
        /// Returns the length of unread data in the packet.
        /// </summary>
        public int UnreadLength()
        {
            return Length() - readPos;
        }

        /// <summary>
        /// Resets the packet. Defaults to true, reset the whole packet. False resets the last read int.
        /// </summary>
        /// <param name="fullReset">Determines if the whole packet should be reset.</param>
        public void Reset(bool fullReset = true)
        {
            if(fullReset)
            {
                buffer.Clear();
                readableBuffer = null;
                readPos = 0;
            }
            else
            {
                readPos -= 4; // "Unread" the last read int.
            }
        }
        #endregion

        #region Write Functions
        public void Write(byte data)
        {
            buffer.Add(data);
        }

        public void Write(byte[] data)
        {
            buffer.AddRange(data);
        }

        public void Write(short data)
        {
            buffer.AddRange(BitConverter.GetBytes(data));
        }

        public void Write(float data)
        {
            buffer.AddRange(BitConverter.GetBytes(data));
        }

        public void Write(bool data)
        {
            buffer.AddRange(BitConverter.GetBytes(data));
        }

        public void Write(string data)
        {
            Write(data.Length);
            buffer.AddRange(Encoding.ASCII.GetBytes(data));
        }
        #endregion

        #region Read Functions
        /// <summary>
        /// Reads a byte from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented.</param>
        /// <returns>Returns the byte that was read.</returns>
        public byte ReadByte(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                byte data = readableBuffer[readPos]; // Get the byte at the current readPos.

                if(moveReadPos)
                {
                    readPos++;
                }

                return data; // Return the byte.
            }
            else
            {
                throw new Exception("Could not read value of type 'byte'!");
            }
        }

        /// <summary>
        /// Reads a range of bytes from the packet.
        /// </summary>
        /// <param name="length">The length of the array to read.</param>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by the length</param>
        /// <returns>Returns the range of bytes that were read.</returns>
        public byte[] ReadBytes(int length, bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                byte[] data = buffer.GetRange(readPos, length).ToArray(); // Get a range of bytes starting at the current readPos with a supplied length.

                if(moveReadPos)
                {
                    readPos += length;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'byte[]'!");
            }
        }

        /// <summary>
        /// Reads a short from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by 2.</param>
        /// <returns>Returns the short that was read.</returns>
        public short ReadShort(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                short data = BitConverter.ToInt16(readableBuffer, readPos); // Convert the bytes to a short.

                if(moveReadPos)
                {
                    readPos += 2;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'short'!");
            }
        }

        /// <summary>
        /// Reads an int from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by 4.</param>
        /// <returns>Returns the int that was read.</returns>
        public int ReadInt(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                int data = BitConverter.ToInt32(readableBuffer, readPos); // Convert the bytes to an int.

                if(moveReadPos)
                {
                    readPos += 4;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'int'!");
            }
        }

        /// <summary>
        /// Reads a long from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by 8.</param>
        /// <returns>Returns the long that was read.</returns>
        public long ReadLong(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                long data = BitConverter.ToInt64(readableBuffer, readPos); // Convert the bytes to a long.

                if(moveReadPos)
                {
                    readPos += 8;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'long'!");
            }
        }

        /// <summary>
        /// Reads a float from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by 4.</param>
        /// <returns>Returns the float that was read.</returns>
        public float ReadFloat(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                float data = BitConverter.ToSingle(readableBuffer, readPos); // Convert the bytes to a float.

                if(moveReadPos)
                {
                    readPos += 4;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'float'!");
            }
        }

        /// <summary>
        /// Reads a bool from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by 1.</param>
        /// <returns>Returns the bool that was read.</returns>
        public bool ReadBool(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                bool data = BitConverter.ToBoolean(readableBuffer, readPos); // Convert the bytes to a bool.

                if(moveReadPos)
                {
                    readPos += 1;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'bool'!");
            }
        }

        /// <summary>
        /// Reads a string from the packet.
        /// </summary>
        /// <param name="moveReadPos">If the buffer's read position should be incremented by the length.</param>
        /// <returns>Returns the string that was read.</returns>
        public string ReadString(bool moveReadPos = true)
        {
            // If there are still bytes left unread.
            if(buffer.Count > readPos)
            {
                int length = ReadInt(); // Get the length of the string.
                string data = Encoding.ASCII.GetString(readableBuffer, readPos, length); // Convert the bytes to a string.

                if(moveReadPos)
                {
                    readPos += length;
                }

                return data;
            }
            else
            {
                throw new Exception("Could not read value of type 'string'!");
            }
        }

        private bool disposed;

        protected virtual void Dispose(bool disposing)
        {
            if(!disposed)
            {
                if(disposing)
                {
                    buffer = null;
                    readableBuffer = null;
                    readPos = 0;
                }

                disposed = true;
            }
        }

        public void Dispose()
        {
            Dispose(true);
            GC.SuppressFinalize(this);
        }
        #endregion
    }
}
