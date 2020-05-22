use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mma8x5x::{ic, mode, Mma8x5x, SlaveAddr};

#[allow(unused)]
pub const ADDRESS: u8 = 0x1D;

pub struct Register {}
#[allow(unused)]
impl Register {
    pub const OUT_X_H: u8 = 0x01;
    pub const WHO_AM_I: u8 = 0x0D;
    pub const XYZ_DATA_CFG: u8 = 0x0E;
    pub const CTRL_REG1: u8 = 0x2A;
    pub const CTRL_REG2: u8 = 0x2B;
    pub const OFF_X: u8 = 0x2F;
}

pub struct BitFlags;
#[allow(unused)]
impl BitFlags {
    pub const ASLP_RATE0: u8 = 1 << 6;
    pub const ASLP_RATE1: u8 = 1 << 7;
    pub const ODR0: u8 = 1 << 3;
    pub const ODR1: u8 = 1 << 4;
    pub const ODR2: u8 = 1 << 5;
    pub const MODS0: u8 = 1;
    pub const MODS1: u8 = 1 << 1;
    pub const SLPE: u8 = 1 << 2;
    pub const SMODS0: u8 = 1 << 3;
    pub const SMODS1: u8 = 1 << 4;
}

#[allow(unused)]
pub fn new_mma8451(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8451, mode::Standby> {
    Mma8x5x::new_mma8451(I2cMock::new(transactions), SlaveAddr::Alternative(true))
}

#[allow(unused)]
pub fn new_mma8452(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8452, mode::Standby> {
    Mma8x5x::new_mma8452(I2cMock::new(transactions), SlaveAddr::Alternative(true))
}

#[allow(unused)]
pub fn new_mma8453(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8453, mode::Standby> {
    Mma8x5x::new_mma8453(I2cMock::new(transactions), SlaveAddr::Alternative(true))
}

#[allow(unused)]
pub fn new_mma8652(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8652, mode::Standby> {
    Mma8x5x::new_mma8652(I2cMock::new(transactions))
}

#[allow(unused)]
pub fn new_mma8653(transactions: &[I2cTrans]) -> Mma8x5x<I2cMock, ic::Mma8653, mode::Standby> {
    Mma8x5x::new_mma8653(I2cMock::new(transactions))
}

pub fn destroy<IC, MODE>(sensor: Mma8x5x<I2cMock, IC, MODE>) {
    sensor.destroy().done();
}

#[macro_export]
macro_rules! assert_near {
    ($value:expr, $expected:expr, $epsilon:expr) => {
        assert!(($value - $epsilon) < $expected);
        assert!(($value + $epsilon) > $expected);
    };
}

#[macro_export]
macro_rules! set_test {
    ($name:ident, $create:ident, $register:ident, $expected:expr, $method:ident $(, $arg:expr )*) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[I2cTrans::write(
                ADDRESS,
                vec![Register::$register, $expected],
            )]);
            sensor.$method($($arg,)*).unwrap();
            destroy(sensor);
        }
    };
}
