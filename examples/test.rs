extern crate bmp280;
extern crate failure;
extern crate i2cdev;

use std::thread;
use std::time;

fn main() -> Result<(), failure::Error> {
    let dev = i2cdev::linux::LinuxI2CDevice::new("/dev/i2c-1", 0x77)?;
    let mut bmp = bmp280::Bmp280::new(dev, bmp280::CHIP_ID)?;

    loop {
        let temp = bmp.read_temperature()?;
        println!("Temperature: {}°C", temp);
        thread::sleep(time::Duration::from_secs(1));
    }
}
