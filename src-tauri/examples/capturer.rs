use std::{thread::sleep, time::Duration};

use kiwi_app_lib::commands::frontend::capture;
fn main() {
    println!("start capturer");
    if let Err(e) = capture::run_capturer() {
        println!("{:#?}", e);
    }
    sleep(Duration::from_millis(5000));
    println!("stop capturer");
    let _ = capture::stop_capturer();
    sleep(Duration::from_millis(500000));
}
