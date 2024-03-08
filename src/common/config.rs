use embedded_hal::i2c::{I2c, SevenBitAddress};

use crate::{
    mode,
    register_access::{BitFlags, Register},
    Config, Error, GScale, Mma8x5x, OutputDataRate, PowerMode, ReadMode,
};

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
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

    /// Set output data rate in WAKE mode
    pub fn set_data_rate(&mut self, rate: OutputDataRate) -> Result<(), Error<E>> {
        let bits = self.ctrl_reg1.bits & !(BitFlags::ODR0 | BitFlags::ODR1 | BitFlags::ODR2);
        let mask = match rate {
            OutputDataRate::Hz800 => 0,
            OutputDataRate::Hz400 => BitFlags::ODR0,
            OutputDataRate::Hz200 => BitFlags::ODR1,
            OutputDataRate::Hz100 => BitFlags::ODR1 | BitFlags::ODR0,
            OutputDataRate::Hz50 => BitFlags::ODR2,
            OutputDataRate::Hz12_5 => BitFlags::ODR2 | BitFlags::ODR0,
            OutputDataRate::Hz6_25 => BitFlags::ODR2 | BitFlags::ODR1,
            OutputDataRate::Hz1_56 => BitFlags::ODR2 | BitFlags::ODR1 | BitFlags::ODR0,
        };
        let bits = bits | mask;
        self.write_reg(Register::CTRL_REG1, bits)?;
        self.ctrl_reg1 = Config { bits };
        Ok(())
    }

    /// Set power mode in WAKE mode
    pub fn set_wake_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Error<E>> {
        let bits = self.ctrl_reg2.bits & !(BitFlags::MODS0 | BitFlags::MODS1);
        let mask = match power_mode {
            PowerMode::Normal => 0,
            PowerMode::LowNoiseLowPower => BitFlags::MODS0,
            PowerMode::HighResolution => BitFlags::MODS1,
            PowerMode::LowPower => BitFlags::MODS1 | BitFlags::MODS0,
        };
        let bits = bits | mask;
        self.write_reg(Register::CTRL_REG2, bits)?;
        self.ctrl_reg2 = Config { bits };
        Ok(())
    }

    /// Reset device
    pub fn reset(&mut self) -> Result<(), Error<E>> {
        self.reset_internal()
    }

    /// Enable self-test mode
    pub fn enable_self_test(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl_reg2.with_high(BitFlags::ST);
        self.write_reg(Register::CTRL_REG2, config.bits)?;
        self.ctrl_reg2 = config;
        Ok(())
    }

    /// Disable self-test mode
    pub fn disable_self_test(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl_reg2.with_low(BitFlags::ST);
        self.write_reg(Register::CTRL_REG2, config.bits)?;
        self.ctrl_reg2 = config;
        Ok(())
    }
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    pub(crate) fn reset_internal(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl_reg2.with_high(BitFlags::RST);
        self.write_reg(Register::CTRL_REG2, config.bits)?;
        self.ctrl_reg1 = Config::default();
        self.ctrl_reg2 = Config::default();
        self.ctrl_reg3 = Config::default();
        self.pl_cfg = Config {
            bits: BitFlags::DBCNTM,
        };
        self.xyz_data_cfg = Config::default();
        Ok(())
    }
}
