//! MLX90614-specific functions

use crate::{ic, mode, register_access::BitFlags, Config, Error, Mma8x5x};
use core::marker::PhantomData;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c;

impl<E, I2C> Mma8x5x<I2C, ic::Mma8652, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the MMA8652 device.
    #[cfg(not(feature = "reset_on_initialization"))]
    pub fn new_mma8652(i2c: I2C) -> Self {
        Mma8x5x::new_mma8652_driver(i2c)
    }

    /// Create new instance of the MMA8652 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(feature = "delay")]
    pub fn new_mma8652<D: DelayUs<u32>>(i2c: I2C, delay: &mut D) -> Result<Self, Error<E>> {
        let mut driver = Mma8x5x::new_mma8652_driver(i2c);
        driver.reset(delay)?;
        Ok(driver)
    }

    /// Create new instance of the MMA8652 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(not(feature = "delay"))]
    pub fn new_mma8652(i2c: I2C) -> Result<Self, Error<E>> {
        let driver = Mma8x5x::new_mma8652_driver(i2c);
        driver.reset()?;
        Ok(driver)
    }

    fn new_mma8652_driver(i2c: I2C) -> Self {
        Mma8x5x {
            i2c,
            address: 0x1D,
            xyz_data_cfg: Config::default(),
            ctrl_reg1: Config::default(),
            ctrl_reg2: Config::default(),
            ctrl_reg3: Config::default(),
            pl_cfg: Config {
                bits: BitFlags::DBCNTM,
            },
            _ic: PhantomData,
            _mode: PhantomData,
        }
    }
}

impl<E, I2C> Mma8x5x<I2C, ic::Mma8653, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the MMA8653 device.
    #[cfg(not(feature = "reset_on_initialization"))]
    pub fn new_mma8653(i2c: I2C) -> Self {
        Mma8x5x::new_mma8653_driver(i2c)
    }

    /// Create new instance of the MMA8653 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(feature = "delay")]
    pub fn new_mma8653<D: DelayUs<u32>>(i2c: I2C, delay: &mut D) -> Result<Self, Error<E>> {
        let mut driver = Mma8x5x::new_mma8653_driver(i2c);
        driver.reset(delay)?;
        Ok(driver)
    }

    /// Create new instance of the MMA8653 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(not(feature = "delay"))]
    pub fn new_mma8653(i2c: I2C) -> Result<Self, Error<E>> {
        let driver = Mma8x5x::new_mma8653_driver(i2c);
        driver.reset()?;
        Ok(driver)
    }

    /// Create new instance of the MMA8653 device.
    fn new_mma8653_driver(i2c: I2C) -> Self {
        Mma8x5x {
            i2c,
            address: 0x1D,
            xyz_data_cfg: Config::default(),
            ctrl_reg1: Config::default(),
            ctrl_reg2: Config::default(),
            ctrl_reg3: Config::default(),
            pl_cfg: Config {
                bits: BitFlags::DBCNTM,
            },
            _ic: PhantomData,
            _mode: PhantomData,
        }
    }
}
