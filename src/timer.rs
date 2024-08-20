use core::fmt;
use std::{
    io::{stdout, Write},
    sync::Arc,
    thread,
    time::{Duration, SystemTime},
};

#[derive()]
pub struct Timer {
    duration: u64,
    elapsed: u64,
}

impl Timer {
    pub fn new(duration: u64, elapsed: u64) -> Self {
        Self { duration, elapsed }
    }

    pub fn start_timer_thread(self: Arc<Self>) {
        let worker = self.clone();
        thread::spawn(move || {
            worker.start_timer();
        });
    }

    pub fn start_timer(&mut self: Arc<Self>) {
        let mut stdout = stdout();
        let timer_duration = Duration::new(self.duration, 0);
        let start = SystemTime::now();

        loop {
            if let Ok(elapsed) = start.elapsed() {
                self.elapsed = elapsed.as_secs();
                print!("\rTimer: {}", self);
                stdout.flush().unwrap();
                if elapsed > timer_duration {
                    return;
                }
            }
            thread::sleep(Duration::from_secs(1));
        }
    }
}

impl fmt::Display for Timer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hours = self.elapsed / 3600;
        let minutes = (self.elapsed - hours * 3600) / 60;
        let seconds = ((self.elapsed - hours * 3600) - (minutes * 60)) % 60;

        if hours < 10 {
            write!(f, "0{}:", hours)?;
        } else {
            write!(f, "{}:", hours)?;
        }

        if minutes < 10 {
            write!(f, "0{}:", minutes)?;
        } else {
            write!(f, "{}:", minutes)?;
        }

        if seconds < 10 {
            write!(f, "0{}", seconds)
        } else {
            write!(f, "{}", seconds)
        }
    }
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     #[test]
//     fn test_timer_constructor() {
//         let timer = Timer::new(3600);
//         assert_eq!((1, 0, 0), (timer.h, timer.m, timer.s));

//         let timer = Timer::new(3601);
//         assert_eq!((1, 0, 1), (timer.h, timer.m, timer.s));

//         let timer = Timer::new(3660);
//         assert_eq!((1, 1, 0), (timer.h, timer.m, timer.s));

//         let timer = Timer::new(3661);
//         assert_eq!((1, 1, 1), (timer.h, timer.m, timer.s));

//         let timer = Timer::new(661);
//         assert_eq!((0, 11, 1), (timer.h, timer.m, timer.s));
//     }
// }
