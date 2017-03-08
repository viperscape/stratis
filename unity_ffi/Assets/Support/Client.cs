
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;

namespace Support
{
    class Client: IDisposable
    {
        internal IntPtr client { get; private set; }
        internal IntPtr rx { get; private set; }

        bool disposed = false;

        [DllImport("stratis_ffi")]
        static extern MClient default_client();

        [DllImport("stratis_ffi")]
        static extern void drop_mclient(MClient mclient);

        // -- //

        [DllImport("stratis_ffi")]
        static extern void get_client_base(IntPtr cptr, [In][Out] ref MClientBase cb);

        public MClientBase GetBase()
        {
            MClientBase cb = new MClientBase();
            get_client_base(client, ref cb);

            return cb;
        }

        public KeyValuePair<byte[],string> GetChat()
        {
            Chat.MChatFrame chat = new Chat.MChatFrame();
            ushort len = Chat.get_client_chat(client, ref chat);
            return chat.GetMsg(len);
        }

        [DllImport("stratis_ffi")]
        public static extern byte client_connect(IntPtr cptr, String s);
        [DllImport("stratis_ffi")]
        public static extern byte client_disconnect(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern byte client_login(IntPtr cptr);
        [DllImport("stratis_ffi")]
        public static extern void client_register(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern byte client_save(IntPtr cptr);
        [DllImport("stratis_ffi")]
        public static extern byte client_load(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern void client_nick(IntPtr cptr, String s);


        [DllImport("stratis_ffi")]
        public static extern float get_client_ping(IntPtr cptr);

        [DllImport("stratis_ffi")]
        public static extern byte is_client_connected(IntPtr cptr);

       

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
        public struct MClient
        { 
            internal IntPtr client;
            internal IntPtr rx; // receiving channel
        }



        public Client ()
        {
            MClient mc = new MClient();
            mc = default_client();
            client = mc.client;
            rx = mc.rx;
        }

        public void Dispose()
        {
            if (!disposed)
            {
                disposed = true;

                MClient mc = new MClient();
                mc.rx = rx;
                mc.client = client;
                drop_mclient(mc);
            }
        }

        ~Client() { Dispose(); }

        public static implicit operator IntPtr(Client c)
        {
            return c.client;
        }
    }
}
