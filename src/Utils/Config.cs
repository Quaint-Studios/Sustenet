using System;
using System.Collections.Generic;
using System.Configuration;
using System.Text;

namespace Sustenet.Utils
{
    /// <summary>
    /// Loads confurations.
    /// </summary>
    class Config
    {
        public static NameValueConfigurationCollection GetConfig(string section)
        {
            var config = ConfigurationManager.GetSection("MasterServer") as NameValueConfigurationCollection;
            Console.WriteLine(config);
            return config;
        }
    }
}
