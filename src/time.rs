use core::time::Duration;

use windows_sys::Win32::System::Performance::{QueryPerformanceCounter, QueryPerformanceFrequency};

unsafe fn ticks() -> i64 {
    let mut count = 0i64;
    QueryPerformanceCounter(&mut count as *mut i64);
    count
}

pub unsafe fn elapsed() -> Duration {
    static mut FREQ: i64 = 0;
    if FREQ == 0 {
        QueryPerformanceFrequency(&mut FREQ as *mut i64);
    }

    static mut START: i64 = 0;
    if START == 0 {
        START = ticks();
    }

    Duration::from_secs_f64((ticks() - START) as f64 / FREQ as f64)
}
