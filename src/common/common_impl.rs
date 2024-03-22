use crate::{register_access::Register, Error, Mma8x5x};
use embedded_hal::i2c::{I2c, SevenBitAddress};

impl<I2C, IC, MODE> Mma8x5x<I2C, IC, MODE> {
    /// Destroy driver instance, return I²C bus and delay instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// Return device ID (Who am I)
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_reg(Register::WHO_AM_I)
    }

    /// Get offset correction for axes X, Y and Z.
    pub fn offset_correction(&mut self) -> Result<(i8, i8, i8), Error<E>> {
        let mut data = [0; 3];
        self.i2c
            .write_read(self.address, &[Register::OFF_X], &mut data)
            .map_err(Error::I2C)?;
        Ok((data[0] as i8, data[1] as i8, data[2] as i8))
    }
}
