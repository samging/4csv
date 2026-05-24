use std::thread::sleep;
use std::time::{Duration, Instant};


fn set_timer() -> Instant {
    Instant::now()
}

fn gettimed() {
    let time_f = set_timer(); 
    sleep(Duration::from_millis(300));
    let time_t = set_timer();
    let calcualte_time = |from:Instant, to: Instant| { to.duration_since(from).as_millis()};
    
    if calcualte_time(time_f, time_t) > 100 {
        println!("Acitvate");
    }
    
    println!("{:?}", calcualte_time(time_f, time_t));
}


fn main() {
    gettimed();
}