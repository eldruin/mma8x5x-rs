use crate::{
    ic, mode,
    register_access::{BitFlags, Register},
    DebounceCounterMode, Error, Mma8x5x,
};
use embedded_hal::blocking::i2c;

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Enable portrait/landscape detection
    pub fn enable_portrait_landscape_detection(&mut self) -> Result<(), Error<E>> {
        let config = self.pl_cfg.with_high(BitFlags::PL_EN);
        self.write_reg(Register::PL_CFG, config.bits)?;
        self.pl_cfg = config;
        Ok(())
    }

    /// Disable portrait/landscape detection
    pub fn disable_portrait_landscape_detection(&mut self) -> Result<(), Error<E>> {
        let config = self.pl_cfg.with_low(BitFlags::PL_EN);
        self.write_reg(Register::PL_CFG, config.bits)?;
        self.pl_cfg = config;
        Ok(())
    }

    /// Set portrait/landscape debounce counter mode
    pub fn set_debounce_counter_mode(&mut self, mode: DebounceCounterMode) -> Result<(), Error<E>> {
        let config = match mode {
            DebounceCounterMode::Decrement => self.pl_cfg.with_low(BitFlags::DBCNTM),
            DebounceCounterMode::Clear => self.pl_cfg.with_high(BitFlags::DBCNTM),
        };
        self.write_reg(Register::PL_CFG, config.bits)?;
        self.pl_cfg = config;
        Ok(())
    }

    /// Set portrait/landscape debounce counter
    pub fn set_debounce_counter(&mut self, counter: u8) -> Result<(), Error<E>> {
        self.set_debounce_counter_internal(counter)
    }
}

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    pub(crate) fn set_debounce_counter_internal(&mut self, counter: u8) -> Result<(), Error<E>> {
        self.write_reg(Register::PL_COUNT, counter)
    }
}

macro_rules! set_allowed_in_active_mode {
    ($ic:ident) => {
        impl<E, I2C> Mma8x5x<I2C, ic::$ic, mode::Active>
        where
            I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
        {
            /// Set portrait/landscape debounce counter
            pub fn set_debounce_counter(&mut self, counter: u8) -> Result<(), Error<E>> {
                self.set_debounce_counter_internal(counter)
            }
        }
    };
}

// Only these two models allow changing these registers in active mode
set_allowed_in_active_mode!(Mma8451);
set_allowed_in_active_mode!(Mma8652);
