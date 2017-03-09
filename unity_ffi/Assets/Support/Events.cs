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
        internal const byte BYTE_LEN = FFI.ID_LEN + 1;
        public byte[] ev = new byte[BYTE_LEN]; // we're expecting an opcode and an id

        public bool has_event = false;

        [DllImport("stratis_ffi")]
        static extern byte poll_event(IntPtr rx, byte[] ev);

        public Events(Client client)
        {
            has_event = (MBool)poll_event(client.rx, ev);
        }

        public Event GetEvent ()
        {
            return (Event)ev[0];
        }

        public byte[] GetId ()
        {
            byte[] r = new byte[BYTE_LEN];
            Buffer.BlockCopy(ev, 1, r, 0, FFI.ID_LEN);
            return r;
        }

        public enum Event : byte {
            Chat = 2,
            Player = 3,
            PlayerDrop = 4
        };
    }
}
