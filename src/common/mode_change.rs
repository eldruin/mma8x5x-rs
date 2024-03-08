use embedded_hal::i2c::{SevenBitAddress, I2c};

use crate::{
    mode,
    register_access::{BitFlags, Register},
    Config, Mma8x5x, ModeChangeError,
};
use core::marker::PhantomData;

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Standby>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// Change mode to active
    pub fn into_active(
        mut self,
    ) -> Result<Mma8x5x<I2C, IC, mode::Active>, ModeChangeError<E, Self>> {
        let config = self.ctrl_reg1.with_high(BitFlags::ACTIVE);
        match self.write_reg(Register::CTRL_REG1, config.bits) {
            Err(error) => Err(ModeChangeError { error, dev: self }),
            Ok(_) => Ok(Mma8x5x {
                i2c: self.i2c,
                address: self.address,
                ctrl_reg1: config,
                ctrl_reg2: self.ctrl_reg2,
                ctrl_reg3: self.ctrl_reg3,
                pl_cfg: self.pl_cfg,
                xyz_data_cfg: self.xyz_data_cfg,
                _ic: PhantomData,
                _mode: PhantomData,
            }),
        }
    }
}

impl<E, I2C, IC> Mma8x5x<I2C, IC, mode::Active>
where
    I2C: I2c<SevenBitAddress, Error = E>,
{
    /// Change mode to standby
    pub fn into_standby(
        mut self,
    ) -> Result<Mma8x5x<I2C, IC, mode::Standby>, ModeChangeError<E, Self>> {
        let config = self.ctrl_reg1.with_low(BitFlags::ACTIVE);
        match self.write_reg(Register::CTRL_REG1, config.bits) {
            Err(error) => Err(ModeChangeError { error, dev: self }),
            Ok(_) => Ok(Mma8x5x {
                i2c: self.i2c,
                address: self.address,
                ctrl_reg1: config,
                ctrl_reg2: self.ctrl_reg2,
                ctrl_reg3: self.ctrl_reg3,
                pl_cfg: self.pl_cfg,
                xyz_data_cfg: self.xyz_data_cfg,
                _ic: PhantomData,
                _mode: PhantomData,
            }),
        }
    }

    /// Reset (changes mode to standby)
    pub fn reset(mut self) -> Result<Mma8x5x<I2C, IC, mode::Standby>, ModeChangeError<E, Self>> {
        match self.reset_internal() {
            Err(error) => Err(ModeChangeError { error, dev: self }),
            Ok(_) => Ok(Mma8x5x {
                i2c: self.i2c,
                address: self.address,
                ctrl_reg1: Config::default(),
                ctrl_reg2: Config::default(),
                ctrl_reg3: Config::default(),
                pl_cfg: Config {
                    bits: BitFlags::DBCNTM,
                },
                xyz_data_cfg: Config::default(),
                _ic: PhantomData,
                _mode: PhantomData,
            }),
        }
    }
}
