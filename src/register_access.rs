use crate::{Error, Mma8x5x};
use embedded_hal::i2c::{I2c, SevenBitAddress};

pub struct Register {}
impl Register {
    pub const STATUS: u8 = 0x00;
    pub const OUT_X_H: u8 = 0x01;
    pub const SYSMOD: u8 = 0x0B;
    pub const INT_SOURCE: u8 = 0x0C;
    pub const WHO_AM_I: u8 = 0x0D;
    pub const XYZ_DATA_CFG: u8 = 0x0E;
    pub const PL_CFG: u8 = 0x11;
    pub const PL_COUNT: u8 = 0x12;
    pub const FF_MT_CFG: u8 = 0x15;
    pub const FF_MT_SRC: u8 = 0x16;
    pub const FF_MT_THS: u8 = 0x17;
    pub const FF_MT_COUNT: u8 = 0x17;
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
impl BitFlags {
    pub const FS0: u8 = 1;
    pub const FS1: u8 = 1 << 1;

    pub const ACTIVE: u8 = 1;
    pub const F_READ: u8 = 1 << 1;
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

    pub const XEFE: u8 = 1 << 3;
    pub const YEFE: u8 = 1 << 4;
    pub const ZEFE: u8 = 1 << 5;
    pub const OAE: u8 = 1 << 6;
    pub const ELE: u8 = 1 << 7;

    pub const XHP: u8 = 1 << 0;
    pub const XHE: u8 = 1 << 1;
    pub const YHP: u8 = 1 << 2;
    pub const YHE: u8 = 1 << 3;
    pub const ZHP: u8 = 1 << 4;
    pub const ZHE: u8 = 1 << 5;
    pub const EA: u8 = 1 << 7;

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

    pub const WAKE_FF_MT: u8 = 1 << 3;
    pub const WAKE_PULSE: u8 = 1 << 4;
    pub const WAKE_LNDPRT: u8 = 1 << 5;
    pub const WAKE_TRANS: u8 = 1 << 6;
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    pub(crate) fn read_reg(&mut self, register: u8) -> Result<u8, Error<E>> {
        let mut data = [0];
        self.i2c
            .write_read(self.address, &[register], &mut data)
            .map_err(Error::I2C)
            .and(Ok(data[0]))
    }
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    pub(crate) fn write_reg(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, value])
            .map_err(Error::I2C)
    }
}
