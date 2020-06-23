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

namespace Sustenet.Events
{
    using System;

    public class BaseEvent
    {
        public event Action Run = delegate { };

        internal void ClearEvents()
        {
            Run = delegate
            { };
        }

        public void RaiseEvent()
        {
            Run();
        }
    }

    public class BaseEvent<T>
    {
        public event Action<T> Run = delegate { };

        internal void ClearEvents()
        {
            Run = delegate
            { };
        }

        public void RaiseEvent(T args)
        {
            Run(args);
        }
    }

    public class BaseEvent<T1, T2>
    {
        public event Action<T1, T2> Run = delegate { };

        internal void ClearEvents()
        {
            Run = delegate
            { };
        }

        public void RaiseEvent(T1 args1, T2 args2)
        {
            Run(args1, args2);
        }
    }
}
