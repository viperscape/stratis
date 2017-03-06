using System;
using System.Runtime.InteropServices;

namespace Assets
{
    class Timer
    {
        [DllImport("stratis_ffi")]
        public static extern IntPtr new_timer(UInt16 time);

        [DllImport("stratis_ffi")]
        public static extern byte drop_timer(IntPtr timer);

        [DllImport("stratis_ffi")]
        public static extern void timer_restart(IntPtr timer);

        [DllImport("stratis_ffi")]
        public static extern byte timer_tick(IntPtr timer);
    }
}
