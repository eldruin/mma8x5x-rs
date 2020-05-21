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
}
