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

namespace Sustenet.Utils
{
    using System.Linq;
    using System.Security.Cryptography;
    using System.IO;
    using System.Xml.Serialization;
    using System.Reflection;

    public static class Security
    {

        public class PasswordOptions
        {
            public int RequiredLength { get; set; } = 8;
            public int RequiredUniqueChars { get; set; } = 4;
            public bool RequireDigit { get; set; } = true;
            public bool RequireNonAlphanumeric { get; set; } = true;
            public bool RequireUppercase { get; set; } = true;
            public bool RequireLowercase { get; set; } = true;
        }

        /// <summary>
        /// Generates a random passphrase that is between 128 and 156 characters long.
        /// </summary>
        /// <returns></returns>
        public static string GeneratePassphrase(PasswordOptions opts = null)
        {
            if(opts == null)
                opts = new PasswordOptions()
                {
                    RequiredLength = RandomNumberGenerator.GetInt32(128, 157),
                    RequiredUniqueChars = 10
                };

            string[] randomChars = new[] {
                "ABCDEFGHJKLMNOPQRSTUVWXYZ",    // uppercase 
                "abcdefghijkmnopqrstuvwxyz",    // lowercase
                "0123456789",                   // digits
                "!@$?_-"                        // non-alphanumeric
            };
            List<char> chars = new List<char>();

            // Uppercase
            if(opts.RequireUppercase)
                chars.Insert(RandomNumberGenerator.GetInt32(chars.Count),
                randomChars[0][RandomNumberGenerator.GetInt32(randomChars[0].Length)]);

            // Lowercase
            if(opts.RequireLowercase)
                chars.Insert(RandomNumberGenerator.GetInt32(chars.Count),
                randomChars[1][RandomNumberGenerator.GetInt32(randomChars[1].Length)]);

            // Numerical
            if(opts.RequireDigit)
                chars.Insert(RandomNumberGenerator.GetInt32(chars.Count),
                randomChars[2][RandomNumberGenerator.GetInt32(randomChars[2].Length)]);

            // Symbols
            if(opts.RequireNonAlphanumeric)
                chars.Insert(RandomNumberGenerator.GetInt32(chars.Count),
                randomChars[3][RandomNumberGenerator.GetInt32(randomChars[3].Length)]);

            for(int i = chars.Count; i < opts.RequiredLength || chars.Distinct().Count() < opts.RequiredUniqueChars; i++)
            {
                string rcs = randomChars[RandomNumberGenerator.GetInt32(randomChars.Length)];
                chars.Insert(RandomNumberGenerator.GetInt32(chars.Count),
                    rcs[RandomNumberGenerator.GetInt32(rcs.Length)]);
            }

            return new string(chars.ToArray());
        }

        public static class Keys
        {
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
            public static void GenerateKeyPair(string keyName, int bit = 2048)
            {
                RSACryptoServiceProvider csp = new RSACryptoServiceProvider(bit);

                RSAParameters privKey = csp.ExportParameters(true);
                RSAParameters pubKey = csp.ExportParameters(false);

                rsaPrivKeys.Add(keyName, privKey);
                rsaPubKeys.Add(keyName, pubKey);

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
                using(StreamWriter writer = new StreamWriter(Path.Combine(Utilities.GetAppPath(), @$"cfg\keys\priv\{keyName}_priv.xml")))
                {
                    serializer.Serialize(writer, privKey);
                }

                // Public Key
                using(StreamWriter writer = new StreamWriter(Path.Combine(Utilities.GetAppPath(), @$"cfg\keys\pub\{keyName}_pub.xml")))
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
            /// Loads all public keys in ./cfg/keys/pub/
            /// </summary>
            /// <param name="serializer">An optional serializer to use.</param>
            public static void LoadPubKeys(XmlSerializer serializer = null)
            {
                string path = Path.Combine(Utilities.GetAppPath(), @$"cfg\keys\pub");

                // Public Key
                foreach(string pubKeyName in Directory.GetFiles(path, "*_pub.xml"))
                {
                    KeyData data = GetKey(path, pubKeyName, KeyType.PublicKey, serializer);
                    rsaPrivKeys.Add(data.name, data.key);
                }
            }

            /// <summary>
            /// Loads all private keys in ./cfg/keys/priv/
            /// </summary>
            /// <param name="serializer">An optional serializer to use.</param>
            public static void LoadPrivKeys(XmlSerializer serializer = null)
            {
                string path = Path.Combine(Utilities.GetAppPath(), @$"cfg\keys\priv");

                // Private Key
                foreach(string privKeyName in Directory.GetFiles(path, "*_priv.xml"))
                {
                    KeyData data = GetKey(path, privKeyName, KeyType.PrivateKey, serializer);
                    rsaPrivKeys.Add(data.name, data.key);
                }
            }

            /// <summary>
            /// Loads a key from a file.
            /// </summary>
            /// <param name="directory">The directory containing the file.</param>
            /// <param name="keyName">The file name without the suffix.</param>
            /// <param name="keyType">The type of key to load.</param>
            /// <param name="serializer">An optional serializer to use.</param>
            /// <returns>The formatted name of the key without any suffixes and the key itself.</returns>
            public static KeyData GetKey(string directory, string keyName, KeyType keyType, XmlSerializer serializer = null)
            {
                try
                {
                    if(serializer == null)
                        serializer = new XmlSerializer(typeof(RSAParameters));

                    string file = Path.Join(directory, keyName);

                    if(!File.Exists(file))
                    {
                        throw new Exception($"{file} does not exist.");
                    }

                    string fileSuffix = keyType == KeyType.PublicKey ? "_pub.xml" : "_priv.xml";

                    using(StreamReader reader = new StreamReader(file))
                    {
                        string formattedName = keyName.Substring(0, keyName.Length - fileSuffix.Length);

                        Console.WriteLine($"Formatted Name: {formattedName}");

                        return new KeyData(formattedName, (RSAParameters)serializer.Deserialize(reader));
                    }
                }
                catch(Exception e)
                {
                    throw new Exception($"Failed to get the key {keyName}: {e}");
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

                return Convert.ToBase64String(csp.Encrypt(dataBytes, false));
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

                byte[] dataBytes = csp.Decrypt(Convert.FromBase64String(data), false);

                return Encoding.Unicode.GetString(dataBytes);
            }
        }
    }
}
