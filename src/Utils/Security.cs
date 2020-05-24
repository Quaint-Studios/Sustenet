using System;
using System.Collections.Generic;
using System.Text;

namespace Sustenet.Utils
{
    using System.Linq;
    using System.Security.Cryptography;

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
    }
}
