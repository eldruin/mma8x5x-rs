use crate::{
    register_access::{BitFlags, Register},
    Error, GScale, Measurement, Mma8x5x, UnscaledMeasurement,
};
use embedded_hal::blocking::i2c;

impl<I2C, IC> Mma8x5x<I2C, IC> {
    /// Destroy driver instance, return I²C bus and delay instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<E, I2C, IC> Mma8x5x<I2C, IC>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Return device ID (Who am I)
    pub fn device_id(&mut self) -> Result<u8, Error<E>> {
        self.read_reg(Register::WHO_AM_I)
    }

    /// Set G scale: +/-2g, +/-4g, +/-8g
    pub fn set_scale(&mut self, scale: GScale) -> Result<(), Error<E>> {
        let config = match scale {
            GScale::G2 => self
                .xyz_data_cfg
                .with_low(BitFlags::FS1)
                .with_low(BitFlags::FS0),
            GScale::G4 => self
                .xyz_data_cfg
                .with_low(BitFlags::FS1)
                .with_high(BitFlags::FS0),
            GScale::G8 => self
                .xyz_data_cfg
                .with_high(BitFlags::FS1)
                .with_high(BitFlags::FS0),
        };
        self.write_reg(Register::XYZ_DATA_CFG, config.bits)?;
        self.xyz_data_cfg = config;
        Ok(())
    }

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
