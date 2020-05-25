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

namespace Sustenet.Tests.Utils.Security
{
    using global::Sustenet.Utils.Security;
    using NUnit.Framework;
    using System.Diagnostics;

    [TestFixture]
    class SecurityTest
    {
        [Test]
        public void GetPassphrase()
        {
            string passphrase = PassphraseGenerator.GeneratePassphrase();

            Debug.WriteLine(passphrase);

            Assert.IsTrue(passphrase.Length > 0, $"Passphrase: {passphrase}");
        }
    }
}
