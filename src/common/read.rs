//! MLX90614-specific functions

use crate::{
    conversion::{convert_10bit, convert_12bit, convert_14bit},
    ic, mode,
    register_access::Register,
    Error, Measurement, Mma8x5x, UnscaledMeasurement,
};
use embedded_hal::blocking::i2c;

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Active>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub(crate) fn scale_measurement(&self, unscaled: UnscaledMeasurement, max: f32) -> Measurement {
        match self.xyz_data_cfg.bits & 0b11 {
            0 => scale(unscaled, max / 2.0),
            1 => scale(unscaled, max / 4.0),
            _ => scale(unscaled, max / 8.0),
        }
    }

    pub(crate) fn read_raw(&mut self) -> Result<(u16, u16, u16), Error<E>> {
        // TODO support 8-bit fast read
        let mut data = [0; 6];
        self.i2c
            .write_read(self.address, &[Register::OUT_X_H], &mut data)
            .map_err(Error::I2C)?;
        Ok((
            (u16::from(data[0]) << 8) | u16::from(data[1]),
            (u16::from(data[2]) << 8) | u16::from(data[3]),
            (u16::from(data[4]) << 8) | u16::from(data[5]),
        ))
    }
}

fn scale(unscaled: UnscaledMeasurement, max: f32) -> Measurement {
    Measurement {
        x: f32::from(unscaled.x) / max,
        y: f32::from(unscaled.y) / max,
        z: f32::from(unscaled.z) / max,
    }
}

macro_rules! read_impl {
    ($ic:ident, $converter:ident, $max:expr) => {
        impl<E, I2C> Mma8x5x<I2C, ic::$ic, mode::Active>
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

read_impl!(Mma8451, convert_14bit, 4096.0);
read_impl!(Mma8452, convert_12bit, 1024.0);
read_impl!(Mma8453, convert_10bit, 256.0);
read_impl!(Mma8652, convert_12bit, 1024.0);
read_impl!(Mma8653, convert_10bit, 256.0);
