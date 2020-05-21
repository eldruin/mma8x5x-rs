use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mma8x5x::{ic, Mma8x5x, SlaveAddr};

#[allow(unused)]
pub const ADDRESS: u8 = 0x1D;

pub struct Register {}
#[allow(unused)]
impl Register {
    pub const OUT_X_H: u8 = 0x01;
    pub const WHO_AM_I: u8 = 0x0D;
    pub const XYZ_DATA_CFG: u8 = 0x0E;
}

#[allow(unused)]
pub fn new_mma8451(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8451> {
    Mma8x5x::new_mma8451(I2cMock::new(transactions), SlaveAddr::Alternative(true))
}

#[allow(unused)]
pub fn new_mma8452(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8452> {
    Mma8x5x::new_mma8452(I2cMock::new(transactions), SlaveAddr::Alternative(true))
}

#[allow(unused)]
pub fn new_mma8453(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8453> {
    Mma8x5x::new_mma8453(I2cMock::new(transactions), SlaveAddr::Alternative(true))
}

#[allow(unused)]
pub fn new_mma8652(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8652> {
    Mma8x5x::new_mma8652(I2cMock::new(transactions))
}

#[allow(unused)]
pub fn new_mma8653(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8653> {
    Mma8x5x::new_mma8653(I2cMock::new(transactions))
}

pub fn destroy<IC>(sensor: Mma8x5x<I2cMock, IC>) {
    sensor.destroy().done();
}

#[macro_export]
macro_rules! assert_near {
    ($value:expr, $expected:expr, $epsilon:expr) => {
        assert!(($value - $epsilon) < $expected);
        assert!(($value + $epsilon) > $expected);
    };
}
