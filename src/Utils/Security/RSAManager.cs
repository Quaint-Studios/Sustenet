

using System.Collections.Generic;
using System.IO;
using System.Security.Cryptography;
using System.Text;
using System.Xml.Serialization;


using System;
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
    public static class RSAManager
    {
        private static string rootPath = @"cfg\keys\rsa";
        private static string privFolder = "priv";
        private static string pubFolder = "pub";
        private static string pubSuffix = "_pub.xml";
        private static string privSuffix = "_priv.xml";

        static RSAManager()
        {
            string privPath = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{privFolder}");

            if(!Directory.Exists(privPath))
                Directory.CreateDirectory(privPath);

            string pubPath = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{pubFolder}");

            if(!Directory.Exists(pubPath))
                Directory.CreateDirectory(pubPath);
        }

        #region RSA Setup
        private static Dictionary<string, RSAParameters> rsaPrivKeys = new Dictionary<string, RSAParameters>();
        private static Dictionary<string, RSAParameters> rsaPubKeys = new Dictionary<string, RSAParameters>();

        public enum KeyType
        {
            PublicKey,
            PrivateKey
        }

        public struct KeyData
        {
            public readonly string name;
            public readonly RSAParameters key;

            public KeyData(string _name, RSAParameters _key)
            {
                name = _name;
                key = _key;
            }
        }

        /// <summary>
        /// Generates an RSA public and private key pair.
        /// </summary>
        /// <param name="keyName">The name to save the key as.</param>
        /// <param name="bit">The bit encryption.</param>
        public static void GenerateKeyPair(string keyName, int bit = 4096)
        {
            RSACryptoServiceProvider csp = new RSACryptoServiceProvider(bit);

            RSAParameters privKey = csp.ExportParameters(true);
            RSAParameters pubKey = csp.ExportParameters(false);

            AddKey(keyName, pubKey, KeyType.PublicKey);
            AddKey(keyName, privKey, KeyType.PrivateKey);

            SaveKeyPair(keyName, privKey, pubKey);
        }

        /// <summary>
        /// Saves a key pair.
        /// </summary>
        /// <param name="keyName">The name of the key.</param>
        /// <param name="privKey">The private key.</param>
        /// <param name="pubKey">The public key.</param>
        public static void SaveKeyPair(string keyName, RSAParameters privKey, RSAParameters pubKey)
        {
            XmlSerializer serializer = new XmlSerializer(typeof(RSAParameters));

            // Private Key
            using(StreamWriter writer = new StreamWriter(Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{privFolder}\{keyName}{privSuffix}")))
            {
                serializer.Serialize(writer, privKey);
            }

            // Public Key
            using(StreamWriter writer = new StreamWriter(Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{pubFolder}\{keyName}{pubSuffix}")))
            {
                serializer.Serialize(writer, pubKey);
            }
        }

        /// <summary>
        /// Loads all public and private keys.
        /// </summary>
        public static void LoadKeys()
        {
            XmlSerializer serializer = new XmlSerializer(typeof(RSAParameters));

            LoadPrivKeys(serializer);
            LoadPubKeys(serializer);
        }

        /// <summary>
        /// Loads all public keys in ./cfg/keys/rsa/pub/
        /// </summary>
        /// <param name="serializer">An optional serializer to use.</param>
        public static void LoadPubKeys(XmlSerializer serializer = null)
        {
            try
            {
                string path = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{pubFolder}");

                // Public Key
                foreach(string pubKeyName in Directory.GetFiles(path, $"*{pubSuffix}"))
                {
                    KeyData data = GetKey(path, Path.GetFileName(pubKeyName), KeyType.PublicKey, serializer);

                    AddKey(data.name, data.key, KeyType.PublicKey);
                }
            }
            catch(Exception e)
            {
                Console.WriteLine(e);
            }
        }

        /// <summary>
        /// Loads all private keys in ./cfg/keys/rsa/priv/
        /// </summary>
        /// <param name="serializer">An optional serializer to use.</param>
        public static void LoadPrivKeys(XmlSerializer serializer = null)
        {
            try
            {
                string path = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{privFolder}");

                // Private Key
                foreach(string privKeyName in Directory.GetFiles(path, $"*{privSuffix}"))
                {
                    KeyData data = GetKey(path, Path.GetFileName(privKeyName), KeyType.PrivateKey, serializer);

                    AddKey(data.name, data.key, KeyType.PrivateKey);
                }
            }
            catch(Exception e)
            {
                Console.WriteLine(e);
            }
        }

        /// <summary>
        /// Loads a single RSA key.
        /// </summary>
        /// <param name="keyName">The name of the key to load.</param>
        /// <param name="keyType">The type of key to load.</param>
        /// <param name="serializer">The serializer to use, if any.</param>
        public static void LoadKey(string keyName, KeyType keyType, XmlSerializer serializer = null)
        {
            try
            {
                // Public Key
                if(keyType == KeyType.PublicKey)
                {
                    string path = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{pubFolder}");
                    KeyData data = GetKey(path, $"{keyName}{pubSuffix}", keyType, serializer);
                    AddKey(data.name, data.key, KeyType.PublicKey);
                    return;
                }

                // Private Key
                if(keyType == KeyType.PrivateKey)
                {
                    string path = Path.Combine(Utilities.GetAppPath(), $@"{rootPath}\{privFolder}");
                    KeyData data = GetKey(path, $"{keyName}{privSuffix}", keyType, serializer);
                    AddKey(data.name, data.key, KeyType.PrivateKey);
                }
            }
            catch(Exception e)
            {
                Console.WriteLine(e);
            }
        }

        private static void AddKey(string name, RSAParameters key, KeyType keyType)
        {
            switch(keyType)
            {
                case KeyType.PublicKey:
                    if(rsaPubKeys.ContainsKey(name))
                    {
                        rsaPubKeys[name] = key;
                    }
                    else
                    {
                        rsaPubKeys.Add(name, key);
                    }
                    return;

                case KeyType.PrivateKey:
                    if(rsaPrivKeys.ContainsKey(name))
                    {
                        rsaPrivKeys[name] = key;
                    }
                    else
                    {
                        rsaPrivKeys.Add(name, key);
                    }
                    return;
            }
        }

        /// <summary>
        /// Loads a key from a file.
        /// </summary>
        /// <param name="directory">The directory containing the file.</param>
        /// <param name="fileName">The file name with the suffix.</param>
        /// <param name="keyType">The type of key to load.</param>
        /// <param name="serializer">An optional serializer to use.</param>
        /// <returns>The formatted name of the key without any suffixes and the key itself.</returns>
        private static KeyData GetKey(string directory, string fileName, KeyType keyType, XmlSerializer serializer = null)
        {
            try
            {
                if(serializer == null)
                    serializer = new XmlSerializer(typeof(RSAParameters));

                string file = Path.Combine(directory, fileName);

                if(!File.Exists(file))
                {
                    throw new Exception($"{file} does not exist.");
                }

                string fileSuffix = keyType == KeyType.PublicKey ? pubSuffix : privSuffix;

                using(StreamReader reader = new StreamReader(file))
                {
                    string formattedName = fileName.Substring(0, fileName.Length - fileSuffix.Length);

                    return new KeyData(formattedName, (RSAParameters)serializer.Deserialize(reader));
                }
            }
            catch(Exception e)
            {
                throw new Exception($"Failed to get the key in {fileName}: {e}");
            }
        }

        /// <summary>
        /// Checks if a key exists in either the pubkey or privkey dictionaries.
        /// </summary>
        /// <param name="keyName">The name of the key to check for.</param>
        /// <param name="keyType">The type of key to check for.</param>
        public static bool KeyExists(string keyName, KeyType keyType = KeyType.PublicKey)
        {
            return keyType == KeyType.PublicKey ? rsaPubKeys.ContainsKey(keyName) : rsaPrivKeys.ContainsKey(keyName);
        }

        /// <summary>
        /// Encrypts a string of data and converts it to Base64.
        /// </summary>
        /// <param name="keyName">The public or private key to use to encrypt data.</param>
        /// <param name="data">The string to encrypt.</param>
        /// <returns>An encrypted base64 string.</returns>
        public static string Encrypt(string keyName, string data)
        {
            RSAParameters? key = null;
            if(rsaPubKeys.ContainsKey(keyName))
            {
                key = rsaPubKeys[keyName];
            }

            // If no public key is found, try to find a private key.
            if(key == null && rsaPrivKeys.ContainsKey(keyName))
            {
                key = rsaPrivKeys[keyName];
            }

            if(key == null)
            {
                throw new Exception($"Failed to find a key that matched '{keyName}'");
            }

            RSACryptoServiceProvider csp = new RSACryptoServiceProvider();
            csp.ImportParameters((RSAParameters)key);

            byte[] dataBytes = Encoding.Unicode.GetBytes(data);

            byte[] cypher = csp.Encrypt(dataBytes, false);

            return Convert.ToBase64String(cypher);
        }

        /// <summary>
        /// Decrypts an encrypted string with the provided key.
        /// </summary>
        /// <param name="keyName">The private key to decrypt with.</param>
        /// <param name="data">The Base64 data to decrypt.</param>
        /// <returns>A decrypted string.</returns>
        public static string Decrypt(string keyName, string data)
        {
            RSAParameters? key = null;
            // Only look for a private key. Because... well.. public keys don't decrypt.
            // .......................At least I hope not.
            if(rsaPrivKeys.ContainsKey(keyName))
            {
                key = rsaPubKeys[keyName];
            }

            if(key == null)
            {
                throw new Exception($"Failed to find a key that matched '{keyName}'");
            }

            RSACryptoServiceProvider csp = new RSACryptoServiceProvider();
            csp.ImportParameters((RSAParameters)key);

            byte[] dataBytes = Convert.FromBase64String(data);

            byte[] passphrase = csp.Decrypt(dataBytes, false);

            return Encoding.Unicode.GetString(passphrase);
        }
        #endregion
    }
}
