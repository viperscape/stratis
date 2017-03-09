using System;
using System.Collections.Generic;
using System.Linq;
using System.Runtime.InteropServices;
using System.Text;
using System.Threading.Tasks;

namespace Support
{
    class Events
    {
        public byte[] ev = new byte[FFI.ID_LEN + 1]; // we're expecting an opcode and an id

        public bool has_event = false;

        [DllImport("stratis_ffi")]
        static extern byte poll_event(IntPtr rx, byte[] ev);

        public Events(Client client)
        {
            has_event = (MBool)poll_event(client.rx, ev);
        }
    }
}
