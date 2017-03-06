
using System;
using System.Runtime.InteropServices;

namespace Assets
{
    class Client: IDisposable
    {
        IntPtr client;
        bool disposed = false;

        [DllImport("stratis_ffi")]
        static extern IntPtr default_client();

        [DllImport("stratis_ffi")]
        static extern byte drop_client(IntPtr cptr);

        // -- //

        [DllImport("stratis_ffi")]
        static extern void get_client_base(IntPtr cptr, [In][Out] ref MClientBase cb);

        public MClientBase getBase()
        {
            MClientBase cb = new MClientBase();
            get_client_base(client, ref cb);

            return cb;
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



        public Client ()
        {
            client = default_client();
        }

        public void Dispose()
        {
            if (!disposed)
            {
                disposed = true;
                drop_client(client);
                Console.WriteLine("dropped client");
            }
        }
    }
}
