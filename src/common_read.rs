//! MLX90614-specific functions

use crate::{conversion::convert_10bit, ic, Error, Measurement, Mma8x5x, UnscaledMeasurement};
use embedded_hal::blocking::i2c;

macro_rules! read_impl {
    ($ic:ident, $converter:ident, $max:expr) => {
        impl<E, I2C> Mma8x5x<I2C, ic::$ic>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Read unscaled acceleration sensor data.
            pub fn read_unscaled(&mut self) -> Result<UnscaledMeasurement, Error<E>> {
                let m = self.read_raw()?;
                Ok($converter(m.0, m.1, m.2))
            }

            /// Read acceleration sensor data scaled to G.
            pub fn read(&mut self) -> Result<Measurement, Error<E>> {
                let unscaled = self.read_unscaled()?;
                Ok(self.scale_measurement(unscaled, $max))
            }
        }
    };
}

read_impl!(Mma8653, convert_10bit, 256.0);
