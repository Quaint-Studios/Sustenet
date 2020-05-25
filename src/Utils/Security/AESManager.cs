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

namespace Sustenet.Utils.Security
{
    using System;
    using System.Collections.Generic;
    using System.IO;
    using System.Security.Cryptography;
    using System.Xml.Serialization;

    public static class AESManager
    {
        private static string rootPath = @"cfg\keys\aes";
        private static string fileSuffix = "_aes.xml";

        static AESManager()
        {
            string keyPath = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}");

            if(!Directory.Exists(keyPath))
                Directory.CreateDirectory(keyPath);
        }

        #region AES Setup
        private static Dictionary<string, byte[]> aesKeys = new Dictionary<string, byte[]>();

        public struct KeyData
        {
            public readonly string name;
            public readonly byte[] key;

            public KeyData(string _name, byte[] _key)
            {
                name = _name;
                key = _key;
            }
        }

        public struct EncryptedData
        {
            /// <summary>
            /// The encrypted string of data as a byte array.
            /// </summary>
            public readonly byte[] cypher;
            /// <summary>
            /// The IV.
            /// </summary>
            public readonly byte[] iv;

            public EncryptedData(byte[] _cypher, byte[] _iv)
            {
                cypher = _cypher;
                iv = _iv;
            }
        }

        /// <summary>
        /// Generates an AES key.
        /// </summary>
        /// <param name="keyName">The name to save the key as.</param>
        /// <param name="bit">The bit encryption.</param>
        public static void GenerateKey(string keyName, int bit = 128)
        {
            AesManaged aes = new AesManaged();

            aes.GenerateKey();

            byte[] aesKey = aes.Key;

            string aesKeyB64 = Convert.ToBase64String(aesKey);

            AddKey(keyName, aesKey);

            SaveAesKey(keyName, aesKeyB64);
        }

        /// <summary>
        /// Saves an AES key.
        /// </summary>
        /// <param name="keyName">The name of the key.</param>
        /// <param name="aesKey">The AES key in Base64 format.</param>
        public static void SaveAesKey(string keyName, string aesKey)
        {
            XmlSerializer serializer = new XmlSerializer(typeof(string));

            // AES Key
            using(StreamWriter writer = new StreamWriter(Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{keyName}{fileSuffix}")))
            {
                serializer.Serialize(writer, aesKey);
            }
        }

        /// <summary>
        /// Loads all AES keys in ./cfg/keys/aes
        /// </summary>
        public static void LoadKeys()
        {
            try
            {
                XmlSerializer serializer = new XmlSerializer(typeof(string));

                string path = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}");

                // Public Key
                foreach(string keyName in Directory.GetFiles(path, $"*{fileSuffix}"))
                {
                    KeyData data = GetKey(path, Path.GetFileName(keyName), serializer);

                    AddKey(data.name, data.key);
                }
            }
            catch(Exception e)
            {
                Console.WriteLine(e);
            }
        }

        /// <summary>
        /// Loads a single AES key.
        /// </summary>
        /// <param name="keyName">The name of the key to load.</param>
        /// <param name="keyType">The type of key to load.</param>
        /// <param name="serializer">The serializer to use, if any.</param>
        public static void LoadKey(string keyName, XmlSerializer serializer = null)
        {
            try
            {
                string path = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}");
                KeyData data = GetKey(path, $"{keyName}{fileSuffix}", serializer);

                AddKey(data.name, data.key);
            }
            catch(Exception e)
            {
                Console.WriteLine(e);
            }
        }


        /// <summary>
        /// Adds a key to the AES dictionary.
        /// </summary>
        /// <param name="name">The name of the key to add.</param>
        /// <param name="key">The key to add.</param>
        public static void AddKey(string name, byte[] key)
        {
            if(aesKeys.ContainsKey(name))
            {
                aesKeys[name] = key;
            }
            else
            {
                aesKeys.Add(name, key);
            }
        }

        /// <summary>
        /// Loads a AES key from a file.
        /// </summary>
        /// <param name="directory">The directory containing the file.</param>
        /// <param name="keyName">The file name without the suffix.</param>
        /// <param name="keyType">The type of key to load.</param>
        /// <param name="serializer">An optional serializer to use.</param>
        /// <returns>The formatted name of the key without any suffixes and the key itself.</returns>
        public static KeyData GetKey(string directory, string keyName, XmlSerializer serializer = null)
        {
            try
            {
                if(serializer == null)
                    serializer = new XmlSerializer(typeof(string));

                string file = Path.Combine(directory, keyName);

                if(!File.Exists(file))
                {
                    throw new Exception($"{file} does not exist.");
                }

                using(StreamReader reader = new StreamReader(file))
                {
                    string formattedName = keyName.Substring(0, keyName.Length - fileSuffix.Length);

                    object key = serializer.Deserialize(reader);

                    return new KeyData(formattedName, Convert.FromBase64String((string)key));
                }
            }
            catch(Exception e)
            {
                throw new Exception($"Failed to get the key {keyName}: {e}");
            }
        }

        /// <summary>
        /// Checks if an AES key exists in the AES folder.
        /// </summary>
        /// <param name="keyName">The name of the AES key to check for.</param>
        public static bool KeyExists(string keyName)
        {
            return aesKeys.ContainsKey(keyName);
        }

        /// <summary>
        /// Encrypts a string of data and converts it to Base64.
        /// </summary>
        /// <param name="keyName">The AES key to use to encrypt data.</param>
        /// <param name="data">The string to encrypt.</param>
        /// <returns>An encrypted base64 string with the IV attached.</returns>
        public static EncryptedData Encrypt(string keyName, string data)
        {
#nullable enable
            byte[]? key = null;
#nullable disable
            if(aesKeys.ContainsKey(keyName))
            {
                key = aesKeys[keyName];
            }

            if(key == null)
            {
                throw new Exception($"Failed to find a key that matched '{keyName}'");
            }

            AesManaged aes = new AesManaged();
            aes.Key = key;
            aes.GenerateIV(); // Generate a unique IV that can be shared but should "never" be reused.

            byte[] encrypted;

            // Encrypt the data
            ICryptoTransform encryptor = aes.CreateEncryptor();
            using(MemoryStream ms = new MemoryStream())
            {
                using(CryptoStream cs = new CryptoStream(ms, encryptor, CryptoStreamMode.Write))
                {
                    using(StreamWriter sw = new StreamWriter(cs))
                    {
                        sw.Write(data);
                    }
                    encrypted = ms.ToArray();
                }
            }

            return new EncryptedData(encrypted, aes.IV);
        }

        /// <summary>
        /// Decrypts an encrypted string with the provided AES key.
        /// </summary>
        /// <param name="keyName">The AES key to decrypt with.</param>
        /// <param name="data">The byte array data to decrypt.</param>
        /// <returns>A decrypted string.</returns>
        public static string Decrypt(string keyName, byte[] data, byte[] iv)
        {
#nullable enable
            byte[]? key = null;
#nullable disable
            if(aesKeys.ContainsKey(keyName))
            {
                key = aesKeys[keyName];
            }

            if(key == null)
            {
                throw new Exception($"Failed to find a key that matched '{keyName}'");
            }

            AesManaged aes = new AesManaged();
            aes.Key = key;
            aes.IV = iv;

            // Decrypt the data
            ICryptoTransform decryptor = aes.CreateDecryptor();
            MemoryStream ms = new MemoryStream(data);
            CryptoStream cs = new CryptoStream(ms, decryptor, CryptoStreamMode.Read);
            StreamReader sr = new StreamReader(cs);

            return sr.ReadToEnd(); // Return the decrypted string.
        }
        #endregion
    }
}
