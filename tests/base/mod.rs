use embedded_hal_mock::i2c::{Mock as I2cMock, Transaction as I2cTrans};
use mma8x5x::{ic, mode, Mma8x5x, SlaveAddr};

#[allow(unused)]
pub const ADDRESS: u8 = 0x1D;

pub struct Register {}
#[allow(unused)]
impl Register {
    pub const STATUS: u8 = 0x00;
    pub const OUT_X_H: u8 = 0x01;
    pub const SYSMOD: u8 = 0x0B;
    pub const INT_SOURCE: u8 = 0x0C;
    pub const WHO_AM_I: u8 = 0x0D;
    pub const XYZ_DATA_CFG: u8 = 0x0E;
    pub const PL_CFG: u8 = 0x11;
    pub const PL_COUNT: u8 = 0x12;
    pub const PL_STATUS: u8 = 0x10;
    pub const ASLP_COUNT: u8 = 0x29;
    pub const CTRL_REG1: u8 = 0x2A;
    pub const CTRL_REG2: u8 = 0x2B;
    pub const CTRL_REG3: u8 = 0x2C;
    pub const CTRL_REG4: u8 = 0x2D;
    pub const CTRL_REG5: u8 = 0x2E;
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
    pub const RST: u8 = 1 << 6;
    pub const ST: u8 = 1 << 7;

    pub const XDR: u8 = 1;
    pub const YDR: u8 = 1 << 1;
    pub const ZDR: u8 = 1 << 2;
    pub const XYZDR: u8 = 1 << 3;
    pub const XOW: u8 = 1 << 4;
    pub const YOW: u8 = 1 << 5;
    pub const ZOW: u8 = 1 << 6;
    pub const XYZOW: u8 = 1 << 7;

    pub const PL_EN: u8 = 1 << 6;
    pub const DBCNTM: u8 = 1 << 7;

    pub const BAFRO: u8 = 1;
    pub const LAPO0: u8 = 1 << 1;
    pub const LAPO1: u8 = 1 << 2;
    pub const LO: u8 = 1 << 6;
    pub const NEWLP: u8 = 1 << 7;

    pub const SRC_DRDY: u8 = 1;
    pub const SRC_FF_MT: u8 = 1 << 2;
    pub const SRC_PULSE: u8 = 1 << 3;
    pub const SRC_LNDPRT: u8 = 1 << 4;
    pub const SRC_TRANS: u8 = 1 << 5;
    pub const SRC_FIFO: u8 = 1 << 6;
    pub const SRC_ASLP: u8 = 1 << 7;

    pub const PP_OD: u8 = 1;
    pub const IPOL: u8 = 1 << 1;

    pub const INT_EN_DRDY: u8 = 1;
    pub const INT_EN_FF_MT: u8 = 1 << 2;
    pub const INT_EN_PULSE: u8 = 1 << 3;
    pub const INT_EN_LNDPRT: u8 = 1 << 4;
    pub const INT_EN_TRANS: u8 = 1 << 5;
    pub const INT_EN_FIFO: u8 = 1 << 6;
    pub const INT_EN_ASLP: u8 = 1 << 7;

    pub const INT_CFG_DRDY: u8 = 1;
    pub const INT_CFG_FF_MT: u8 = 1 << 2;
    pub const INT_CFG_PULSE: u8 = 1 << 3;
    pub const INT_CFG_LNDPRT: u8 = 1 << 4;
    pub const INT_CFG_TRANS: u8 = 1 << 5;
    pub const INT_CFG_FIFO: u8 = 1 << 6;
    pub const INT_CFG_ASLP: u8 = 1 << 7;
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

#[macro_export]
macro_rules! get_test {
    ($name:ident, $create:ident, $register:ident, $read:expr, $method:ident, $expected:expr) => {
        #[test]
        fn $name() {
            let mut sensor = $create(&[I2cTrans::write_read(
                ADDRESS,
                vec![Register::$register],
                vec![$read],
            )]);
            let v = sensor.$method().unwrap();
            assert_eq!(v, $expected);
            destroy(sensor);
        }
    };
}
