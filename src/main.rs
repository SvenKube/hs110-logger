extern crate hs110;

use hs110::SmartPlug;

use std::thread;
use std::time::Duration;

fn main() {
    let plug_ip = std::env::var("HS110_IP").expect("Could not read environment variable: HS110_IP");
    let plug = SmartPlug::new(plug_ip + ":9999");

    let t = thread::spawn(move || {
        loop {
            let emeter_realtime = plug.get_emeter_realtime().unwrap()
                .emeter.unwrap()
                .get_realtime.unwrap();


            println!("Current power: {}", emeter_realtime.power_mw);
            thread::sleep(Duration::from_millis(2000));
        }
    });

    if let Err(error) = t.join() {
        eprintln!("Something went wrong: {:#?}", error);
    }
}
