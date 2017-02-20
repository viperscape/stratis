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


        [DllImport("stratis_unity")]
        public static extern IntPtr new_client();

        [DllImport("stratis_unity")]
        public static extern void drop_client(IntPtr cptr);

        // -- //

        [DllImport("stratis_unity")]
        public static extern void get_client_base(IntPtr cptr, [In][Out] ref MClientBase cb);


        [DllImport("stratis_unity")]
        public static extern IntPtr default_client(Byte[] key, Byte[] id);

        [DllImport("stratis_unity")]
        public static extern bool client_connect(IntPtr cptr, String s);
        [DllImport("stratis_unity")]
        public static extern bool client_login(IntPtr cptr);
        [DllImport("stratis_unity")]
        public static extern void client_register(IntPtr cptr);

        [DllImport("stratis_unity")]
        public static extern bool client_save(IntPtr cptr);
        [DllImport("stratis_unity")]
        public static extern bool client_load(IntPtr cptr);

        [DllImport("stratis_unity")]
        public static extern void client_chat(IntPtr cptr, String s);
        [DllImport("stratis_unity")]
        public static extern void client_nick(IntPtr cptr, String s);

        //[DllImport("stratis_unity")]
        //public static extern bool get_client_chat(IntPtr cptr, ChatFrame chat);
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
}
