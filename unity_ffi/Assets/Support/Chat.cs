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
        static extern UInt16 get_client_chat(IntPtr cptr, byte[] id, byte[] msg);

        public static string GetMsg(Client client, byte[] id)
        {
            byte[] msg = new byte[FFI.MAX_TEXT_LEN];
            ushort len = get_client_chat(client.client, id, msg);
            return System.Text.Encoding.UTF8.GetString(msg, 0, len);
        }
    }
}
