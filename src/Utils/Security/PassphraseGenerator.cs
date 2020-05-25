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
    using System.Linq;
    using System.Security.Cryptography;

    class PassphraseGenerator
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

            string[] randomChars = new string[] {
                "ABCDEFGHJKLMNOPQRSTUVWXYZ",    // uppercase 
                "abcdefghijkmnopqrstuvwxyz",    // lowercase
                "0123456789",                   // digits
                "!@$?_-"                        // non-alphanumeric
            };
            List<char> chars = new List<char>();

            // Uppercase
            if(opts.RequireUppercase)
                chars.Insert(RandomNumberGenerator.GetInt32(Math.Max(chars.Count, 1)),
                randomChars[0][RandomNumberGenerator.GetInt32(randomChars[0].Length)]);

            // Lowercase
            if(opts.RequireLowercase)
                chars.Insert(RandomNumberGenerator.GetInt32(Math.Max(chars.Count, 1)),
                randomChars[1][RandomNumberGenerator.GetInt32(randomChars[1].Length)]);

            // Numerical
            if(opts.RequireDigit)
                chars.Insert(RandomNumberGenerator.GetInt32(Math.Max(chars.Count, 1)),
                randomChars[2][RandomNumberGenerator.GetInt32(randomChars[2].Length)]);

            // Symbols
            if(opts.RequireNonAlphanumeric)
                chars.Insert(RandomNumberGenerator.GetInt32(Math.Max(chars.Count, 1)),
                randomChars[3][RandomNumberGenerator.GetInt32(randomChars[3].Length)]);

            for(int i = chars.Count; i < opts.RequiredLength || chars.Distinct().Count() < opts.RequiredUniqueChars; i++)
            {
                string rcs = randomChars[RandomNumberGenerator.GetInt32(randomChars.Length)];
                chars.Insert(RandomNumberGenerator.GetInt32(Math.Max(chars.Count, 1)),
                    rcs[RandomNumberGenerator.GetInt32(rcs.Length)]);
            }

            return new string(chars.ToArray());
        }
    }
}
