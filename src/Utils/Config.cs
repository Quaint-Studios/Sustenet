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

namespace Sustenet.Utils
{
    using System.Configuration;
    using System.IO;

    /// <summary>
    /// Loads configurations.
    /// </summary>
    class Config
    {
        public enum ConfigType
        {
            MasterServer,
            ClusterServer
        }

        static Config()
        {
            string cfgPath = Path.Combine(Utilities.GetAppPath(), $@"cfg");

            if(!Directory.Exists(cfgPath))
                Directory.CreateDirectory(cfgPath);
        }

        /// <summary>
        /// Loads a custom config file and returns the data associated with it.
        /// </summary>
        /// <param name="section">The name of the custom config file.</param>
        /// <returns>The data associated with the custom config file.</returns>
        public static KeyValueConfigurationCollection GetConfig(ConfigType configType)
        {
            ExeConfigurationFileMap configMap = new ExeConfigurationFileMap();
            configMap.ExeConfigFilename = Path.Combine(Utilities.GetAppPath(), $@"cfg\{configType.ToString()}.config");
            Configuration config = ConfigurationManager.OpenMappedExeConfiguration(configMap, ConfigurationUserLevel.None);

            return config.AppSettings.Settings;
        }
    }
}
