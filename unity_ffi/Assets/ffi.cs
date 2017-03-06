using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;

namespace Assets
{
    public class FFI
    {
        public const byte ID_LEN = 16;
        public const byte KEY_LEN = 20;
        public const UInt16 MAX_TEXT_LEN = 2048;
    }

    // implicit byte transform for marshalling boolean
    public class MBool
    {
        private byte inner;
        public MBool (byte b) { inner = b; }

        public static implicit operator bool(MBool b)
        {
            return Convert.ToBoolean(b.inner);
        }

        public static implicit operator MBool(byte b)
        {
            return new MBool(b);
        }
    }
}
