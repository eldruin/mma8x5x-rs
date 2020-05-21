use crate::{Error, Mma8x5x};
use embedded_hal::blocking::i2c;

pub struct Register {}
impl Register {
    pub const OUT_X_H: u8 = 0x01;
    pub const WHO_AM_I: u8 = 0x0D;
    pub const XYZ_DATA_CFG: u8 = 0x0E;
    pub const CTRL_REG1: u8 = 0x2A;
    pub const OFF_X: u8 = 0x2F;
}

pub struct BitFlags;
impl BitFlags {
    pub const FS1: u8 = 1 << 1;
    pub const FS0: u8 = 1;
    pub const ACTIVE: u8 = 1;
    pub const F_READ: u8 = 1 << 1;
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: i2c::WriteRead<Error = E>,
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
    I2C: i2c::Write<Error = E>,
{
    pub(crate) fn write_reg(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, value])
            .map_err(Error::I2C)
    }
}
