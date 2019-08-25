extern crate hs110;

use hs110::SmartPlug;

use influx_db_client::{Client, Point, Precision, Value};
use std::thread;
use std::time::Duration;

fn main() {
    let influx_username = std::env::var("HS110_INFLUX_USERNAME")
        .expect("Could not read environment variable: HS110_INFLUX_USERNAME");
    let influx_password = std::env::var("HS110_INFLUX_PASSWORD")
        .expect("Could not read environment variable: HS110_INFLUX_PASSWORD");
    let influx_database = std::env::var("HS110_INFLUX_DATABASE")
        .expect("Could not read environment variable: HS110_INFLUX_DATABASE");

    let influx_host = std::env::var("HS110_INFLUX_HOST")
        .expect("Could not read environment variable: HS110_INFLUX_HOST");

    let influx_client = Client::new(influx_host, influx_database)
        .set_authentication(influx_username, influx_password);

    let plug_ip = std::env::var("HS110_IP").expect("Could not read environment variable: HS110_IP");
    let plug = SmartPlug::new(plug_ip + ":9999");

    let t = thread::spawn(move || loop {
        let sysinfo = plug.get_sysinfo().unwrap().system.unwrap().get_sysinfo;

        let emeter_realtime = plug
            .get_emeter_realtime()
            .unwrap()
            .emeter
            .unwrap()
            .get_realtime
            .unwrap();

        let mut point = Point::new("hs110");

        point.add_field("voltage_mv", Value::Integer(emeter_realtime.voltage_mv));
        point.add_field("current_ma", Value::Integer(emeter_realtime.current_ma));
        point.add_field("power_mw", Value::Integer(emeter_realtime.power_mw));
        point.add_field("total_wh", Value::Integer(emeter_realtime.total_wh));

        point.add_tag("device_id", Value::String(sysinfo.device_id));
        point.add_tag("dev_name", Value::String(sysinfo.dev_name));

        let _ = influx_client.write_point(point, Some(Precision::Seconds), None);

        println!("Send new measurement...");
        thread::sleep(Duration::from_millis(2000));
    });

    if let Err(error) = t.join() {
        eprintln!("Something went wrong: {:#?}", error);
    }
}
