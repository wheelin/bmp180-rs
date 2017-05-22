extern crate bmp180_driver;

use bmp180_driver::bmp180::*;

fn main() {
    let mut sensor = BMP180::new();
    let pressure = sensor.read_pressure(Oss::Oss8);
    let temperature = sensor.read_temperature();
}
