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

        [DllImport("stratis_ffi")]
        public static extern IntPtr default_client();

        [DllImport("stratis_ffi")]
        public static extern byte drop_client(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern byte ping_client(IntPtr cptr);

        // -- //

        [DllImport("stratis_ffi")]
        public static extern void get_client_base(IntPtr cptr, [In][Out] ref MClientBase cb);


        [DllImport("stratis_ffi")]
        public static extern byte client_connect(IntPtr cptr, String s);
        [DllImport("stratis_ffi")]
        public static extern byte client_login(IntPtr cptr);
        [DllImport("stratis_ffi")]
        public static extern void client_register(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern byte client_save(IntPtr cptr);
        [DllImport("stratis_ffi")]
        public static extern byte client_load(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern void client_chat(IntPtr cptr, String s);
        [DllImport("stratis_ffi")]
        public static extern void client_nick(IntPtr cptr, String s);

        [DllImport("stratis_ffi")]
        [return: MarshalAs(UnmanagedType.U2)]
        public static extern UInt16 get_client_chat(IntPtr cptr, [In][Out] ref MChatFrame chat);
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct MClientBase
    {
        [MarshalAs(UnmanagedType.ByValArray, 
            ArraySubType = UnmanagedType.U1, SizeConst = FFI.ID_LEN)]
        public byte[] id;

        [MarshalAs(UnmanagedType.ByValArray,
            ArraySubType = UnmanagedType.U1, SizeConst = FFI.KEY_LEN)]
        public byte[] key;
    }

    [StructLayout(LayoutKind.Sequential)]
    public struct MChatFrame
    {
        [MarshalAs(UnmanagedType.ByValArray,
            ArraySubType = UnmanagedType.U1, SizeConst = FFI.ID_LEN)]
        public byte[] id;

        [MarshalAs(UnmanagedType.ByValArray,
            ArraySubType = UnmanagedType.U1, SizeConst = FFI.MAX_TEXT_LEN)]
        public byte[] msg;

        public string get_msg (UInt16 len)
        {
            return System.Text.Encoding.UTF8.GetString(this.msg, 0, len);
        }
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
