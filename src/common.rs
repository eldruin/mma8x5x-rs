use crate::{register_access::Register, Error, Mma8x5x};
use embedded_hal::blocking::i2c;

impl<I2C, IC> Mma8x5x<I2C, IC> {
    /// Destroy driver instance, return I²C bus and delay instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<E, I2C, IC> Mma8x5x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E>,
{
    /// Return device ID (Who am I)
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_reg(Register::WHO_AM_I)
    }
}
