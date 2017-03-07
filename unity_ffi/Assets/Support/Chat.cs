using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Support
{
    class Chat
    {
        [DllImport("stratis_ffi")]
        public static extern void client_chat(IntPtr cptr, String s);

        [DllImport("stratis_ffi")]
        [return: MarshalAs(UnmanagedType.U2)]
        public static extern UInt16 get_client_chat(IntPtr cptr, [In][Out] ref MChatFrame chat);



        [StructLayout(LayoutKind.Sequential)]
        public struct MChatFrame
        {
            [MarshalAs(UnmanagedType.ByValArray,
                ArraySubType = UnmanagedType.U1, SizeConst = FFI.ID_LEN)]
            public byte[] id;

            [MarshalAs(UnmanagedType.ByValArray,
                ArraySubType = UnmanagedType.U1, SizeConst = FFI.MAX_TEXT_LEN)]
            public byte[] msg;

            public KeyValuePair<byte[], string> GetMsg(UInt16 len)
            {
                return new KeyValuePair<byte[], string>
                    (id, 
                     System.Text.Encoding.UTF8.GetString(this.msg, 0, len));
            }
        }
    }
}
