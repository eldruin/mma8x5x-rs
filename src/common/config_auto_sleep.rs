use crate::{
    mode,
    register_access::{BitFlags, Register},
    AutoSleepDataRate, Config, Error, Mma8x5x, PowerMode,
};
use embedded_hal::blocking::i2c;

/// Auto-sleep/wake configuration
impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Set power mode in sleep mode
    pub fn set_sleep_power_mode(&mut self, power_mode: PowerMode) -> Result<(), Error<E>> {
        let bits = self.ctrl_reg2.bits & !(BitFlags::SMODS0 | BitFlags::SMODS1);
        let mask = match power_mode {
            PowerMode::Normal => 0,
            PowerMode::LowNoiseLowPower => BitFlags::SMODS0,
            PowerMode::HighResolution => BitFlags::SMODS1,
            PowerMode::LowPower => BitFlags::SMODS1 | BitFlags::SMODS0,
        };
        self.write_reg(Register::CTRL_REG2, bits | mask)?;
        self.ctrl_reg2 = Config { bits };
        Ok(())
    }

    /// Enable auto-sleep mode
    pub fn enable_auto_sleep(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl_reg2.with_high(BitFlags::SLPE);
        self.write_reg(Register::CTRL_REG2, config.bits)?;
        self.ctrl_reg2 = config;
        Ok(())
    }

    /// Disable auto-sleep mode
    pub fn disable_auto_sleep(&mut self) -> Result<(), Error<E>> {
        let config = self.ctrl_reg2.with_low(BitFlags::SLPE);
        self.write_reg(Register::CTRL_REG2, config.bits)?;
        self.ctrl_reg2 = config;
        Ok(())
    }

    /// Set sampling rate used in auto-sleep mode
    pub fn set_auto_sleep_data_rate(&mut self, rate: AutoSleepDataRate) -> Result<(), Error<E>> {
        let bits = self.ctrl_reg1.bits & !(BitFlags::ASLP_RATE1 | BitFlags::ASLP_RATE0);
        let mask = match rate {
            AutoSleepDataRate::Hz50 => 0,
            AutoSleepDataRate::Hz12_5 => BitFlags::ASLP_RATE0,
            AutoSleepDataRate::Hz6_25 => BitFlags::ASLP_RATE1,
            AutoSleepDataRate::Hz1_56 => BitFlags::ASLP_RATE1 | BitFlags::ASLP_RATE0,
        };
        self.write_reg(Register::CTRL_REG1, bits | mask)?;
        self.ctrl_reg1 = Config { bits };
        Ok(())
    }

    /// Set auto-sleep/wake count
    pub fn set_auto_sleep_count(&mut self, count: u8) -> Result<(), Error<E>> {
        self.write_reg(Register::ASLP_COUNT, count)
    }
}
