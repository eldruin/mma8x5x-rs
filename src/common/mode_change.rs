use crate::{
    mode,
    register_access::{BitFlags, Register},
    Mma8x5x, ModeChangeError,
};
use core::marker::PhantomData;
use embedded_hal::blocking::i2c;

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Change mode to active
    pub fn active(mut self) -> Result<Mma8x5x<I2C, IC, mode::Active>, ModeChangeError<E, Self>> {
        let config = self.ctrl_reg1.with_high(BitFlags::ACTIVE);
        match self.write_reg(Register::CTRL_REG1, config.bits) {
            Err(error) => Err(ModeChangeError { error, dev: self }),
            Ok(_) => Ok(Mma8x5x {
                i2c: self.i2c,
                address: self.address,
                xyz_data_cfg: self.xyz_data_cfg,
                ctrl_reg1: config,
                _ic: PhantomData,
                _mode: PhantomData,
            }),
        }
    }
}

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Active>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Change mode to standby
    pub fn standby(mut self) -> Result<Mma8x5x<I2C, IC, mode::Standby>, ModeChangeError<E, Self>> {
        let config = self.ctrl_reg1.with_low(BitFlags::ACTIVE);
        match self.write_reg(Register::CTRL_REG1, config.bits) {
            Err(error) => Err(ModeChangeError { error, dev: self }),
            Ok(_) => Ok(Mma8x5x {
                i2c: self.i2c,
                address: self.address,
                xyz_data_cfg: self.xyz_data_cfg,
                ctrl_reg1: config,
                _ic: PhantomData,
                _mode: PhantomData,
            }),
        }
    }
}
