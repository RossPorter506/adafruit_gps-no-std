extern crate adafruit_gps;

pub use adafruit_gps::gps::{GetGpsData, Gps, open_port};
use adafruit_gps::PMTK::send_pmtk::SendPmtk;

use std::thread;
use std::time::Duration;

fn main() {
    let port = open_port("/dev/serial0");
    let mut gps = Gps {port};

    gps.pmtk_314_api_set_nmea_output(0,0,1,1,1,1,1);

    loop {
        let values = gps.update();
        let pretty_print = format!("\
        utc: {}\
        lat:  {:?}\
        long: {:?}\
        alt:  {:?}\
        course true:{:?}\
        course mag: {:?}\
        knots: {:?}\
        kph:   {:?}\
        geo: {:?}\
        age: {:?}\
        sats: {:?}\
        hdop: {:?}\
        vdop: {:?}\
        pdop: {:?}\
        satellites: {:?}\
        ", values.utc, values.latitude, values.longitude, values.altitude, values.true_course,
        values.mag_course, values.speed_knots, values.speed_kph, values.geoidal_spe, values.age_diff_corr,
        values.sats_used, values.hdop, values.vdop, values.pdop, values.satellites);
        println!("{}", pretty_print);
        thread::sleep(Duration::from_secs(1))
    }

}