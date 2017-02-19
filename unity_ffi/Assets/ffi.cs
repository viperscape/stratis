using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;

namespace Assets
{
    public class FFI
    {
        [DllImport("stratis_unity")]
        public static extern IntPtr new_client();

        [DllImport("stratis_unity")]
        public static extern void drop_client(IntPtr cptr);

        [DllImport("stratis_unity")]
        public static extern Byte[] get_client_id(IntPtr cptr); // TODO: actually marshall data

        [DllImport("stratis_unity")]
        public static extern Byte[] get_client_key(IntPtr cptr);

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
}
