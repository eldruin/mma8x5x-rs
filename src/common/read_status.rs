//! MLX90614-specific functions

use crate::{
    register_access::{BitFlags as BF, Register},
    DataStatus, Error, Mma8x5x, SystemMode,
};
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

    /// Read current data status
    pub fn data_status(&mut self) -> Result<DataStatus, Error<E>> {
        let st = self.read_reg(Register::STATUS)?;
        Ok(DataStatus {
            xyz_overwrite: (st & BF::XYZOW) != 0,
            z_overwrite: (st & BF::ZOW) != 0,
            y_overwrite: (st & BF::YOW) != 0,
            x_overwrite: (st & BF::XOW) != 0,
            xyz_new_data: (st & BF::XYZDR) != 0,
            z_new_data: (st & BF::ZDR) != 0,
            y_new_data: (st & BF::YDR) != 0,
            x_new_data: (st & BF::XDR) != 0,
        })
    }
}
