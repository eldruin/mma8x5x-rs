use crate::{Error, Mma8x5x};
use embedded_hal::blocking::i2c;

pub struct Register {}
impl Register {
    pub const OUT_X_H: u8 = 0x01;
    // pub const OUT_X_L: u8 = 0x02;
    // pub const OUT_Y_H: u8 = 0x03;
    // pub const OUT_Y_L: u8 = 0x04;
    // pub const OUT_Z_H: u8 = 0x05;
    // pub const OUT_Z_L: u8 = 0x06;
    pub const WHO_AM_I: u8 = 0x0D;
}

impl<E, I2C, IC> Mma8x5x<I2C, IC>
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

impl<E, I2C, IC> Mma8x5x<I2C, IC>
where
    I2C: i2c::Write<Error = E>,
{
    pub(crate) fn write_reg(&mut self, register: u8, value: u8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[register, value])
            .map_err(Error::I2C)
    }
}
