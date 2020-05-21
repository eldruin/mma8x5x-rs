use crate::{
    register_access::{BitFlags, Register},
    Error, GScale, Mma8x5x, ReadMode,
};
use embedded_hal::blocking::i2c;

impl<I2C, IC, MODE> Mma8x5x<I2C, IC, MODE> {
    /// Destroy driver instance, return I²C bus and delay instance.
    pub fn destroy(self) -> I2C {
        self.i2c
    }
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
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
                .with_low(BitFlags::FS0),
        };
        self.write_reg(Register::XYZ_DATA_CFG, config.bits)?;
        self.xyz_data_cfg = config;
        Ok(())
    }

    /// Set read mode (Normal/Fast)
    pub fn set_read_mode(&mut self, mode: ReadMode) -> Result<(), Error<E>> {
        let config = match mode {
            ReadMode::Normal => self.ctrl_reg1.with_low(BitFlags::F_READ),
            ReadMode::Fast => self.ctrl_reg1.with_high(BitFlags::F_READ),
        };
        self.write_reg(Register::CTRL_REG1, config.bits)?;
        self.ctrl_reg1 = config;
        Ok(())
    }

    /// Set offset correction.
    ///
    /// The resolution is 1.96/LSB. The offset compensation range is +/-250mg.
    pub fn set_offset_correction(&mut self, x: i8, y: i8, z: i8) -> Result<(), Error<E>> {
        self.i2c
            .write(self.address, &[Register::OFF_X, x as u8, y as u8, z as u8])
            .map_err(Error::I2C)
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
