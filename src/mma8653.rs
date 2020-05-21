//! MLX90614-specific functions

use crate::{
    conversion::convert_10bit, ic, register_access::Register, Error, Mma8x5x, UnscaledMeasurement,
};
use embedded_hal::blocking::i2c;

impl<E, I2C> Mma8x5x<I2C, ic::Mma8653>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Read unscaled acceleration sensor data.
    pub fn read_unscaled(&mut self) -> Result<UnscaledMeasurement, Error<E>> {
        // TODO support 8-bit fast read
        let mut data = [0; 6];
        self.i2c
            .write_read(self.address, &[Register::OUT_X_H], &mut data)
            .map_err(Error::I2C)?;
        let m = convert_10bit(
            (u16::from(data[0]) << 8) | u16::from(data[1]),
            (u16::from(data[2]) << 8) | u16::from(data[3]),
            (u16::from(data[4]) << 8) | u16::from(data[5]),
        );
        Ok(m)
    }
}
