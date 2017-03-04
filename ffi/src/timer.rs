use std::time::{Instant};

/// a timer to track elapsed seconds

pub struct Timer {
    time: u16,
    start: Instant,
}

impl Timer {
    pub fn new (t: u16) -> Timer {
        Timer { time: t,
                start: Instant::now() }
    }

    pub fn restart(&mut self) {
        self.start = Instant::now();
    }

    pub fn tick (&self) -> bool {
        self.start.elapsed().as_secs() >= self.time as u64
    }
}

//
#[no_mangle]
pub extern fn new_timer (t: u16) -> *mut Timer {
    Box::into_raw(Box::new(Timer::new(t)))
}

#[no_mangle]
pub extern fn drop_timer (timer: *mut Timer) -> u8 {
    if !timer.is_null() {
        unsafe { Box::from_raw(timer); }
    }

    timer.is_null() as u8
}

#[no_mangle]
pub extern fn timer_restart (timer: *mut Timer) {
    let mut timer = unsafe { &mut *timer };
    timer.restart();
}

#[no_mangle]
pub extern fn timer_tick (timer: *mut Timer) -> u8 {
    let timer = unsafe { & *timer };
    timer.tick() as u8
}
