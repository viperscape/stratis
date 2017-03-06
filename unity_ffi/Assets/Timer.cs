﻿using System;
using System.Runtime.InteropServices;

namespace Assets
{
    class Timer: IDisposable
    {
        IntPtr timer;

        [DllImport("stratis_ffi")]
        static extern IntPtr new_timer(UInt16 time);

        [DllImport("stratis_ffi")]
        static extern byte drop_timer(IntPtr timer);

        [DllImport("stratis_ffi")]
        static extern void timer_restart(IntPtr timer);

        [DllImport("stratis_ffi")]
        static extern byte timer_tick(IntPtr timer);

        public void Dispose()
        {
            drop_timer(timer);
        }

        ~Timer() { Dispose(); }

        public Timer(UInt16 t)
        {
            timer = new_timer(t);
        }

        public void restart () { timer_restart(timer); }
        public MBool tick() { return timer_tick(timer); }
    }
}
