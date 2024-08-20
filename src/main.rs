use crate::timer::Timer;

mod timer;

fn main() {
    println!("Hello, world!");
    let mut timer = Timer::new(3600, 0);
    timer.start_timer();
}
