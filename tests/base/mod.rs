use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mma8x5x::{ic, Mma8x5x};

#[allow(unused)]
pub struct Register {}
impl Register {
    pub const WHO_AM_I: u8 = 0x0D;
}

#[allow(unused)]
pub fn new_mma8653(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8653> {
    Mma8x5x::new_mma8653(I2cMock::new(transactions))
}

pub fn destroy<IC>(sensor: Mma8x5x<I2cMock, IC>) {
    sensor.destroy().done();
}
