//! MLX90614-specific functions

use crate::{register_access::Register, Error, Mma8x5x, SystemMode};
use embedded_hal::blocking::i2c;

impl<E, I2C, IC, MODE> Mma8x5x<I2C, IC, MODE>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Read current system mode
    pub fn system_mode(&mut self) -> Result<SystemMode, Error<E>> {
        let sysmod = self.read_reg(Register::SYSMOD)?;
        match sysmod & 0b11 {
            0 => Ok(SystemMode::Standby),
            1 => Ok(SystemMode::Wake),
            _ => Ok(SystemMode::Sleep),
        }
    }
}
