use linux_embedded_hal::{Delay, I2cdev};
use mma8x5x::Mma8x5x;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let sensor = Mma8x5x::new_mma8653(dev, &mut Delay).unwrap();
    let mut sensor = sensor.into_active().ok().unwrap();
    loop {
        let accel = sensor.read().unwrap();
        println!("Acceleration: {:?}", accel);
    }
}
