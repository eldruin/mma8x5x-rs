use crate::{
    ic, mode, register_access::BitFlags, Config, Error, Mma8x5x, SlaveAddr, MMA845X_BASE_ADDR,
};
use core::marker::PhantomData;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c;

impl<E, I2C> Mma8x5x<I2C, ic::Mma8451, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the MMA8451 device.
    #[cfg(not(feature = "reset_on_initialization"))]
    pub fn new_mma8451(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x::new_mma8451_driver(i2c, address)
    }

    /// Create new instance of the MMA8451 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(feature = "delay")]
    pub fn new_mma8451<D: DelayUs<u32>>(
        i2c: I2C,
        address: SlaveAddr,
        delay: &mut D,
    ) -> Result<Self, Error<E>> {
        let mut driver = Mma8x5x::new_mma8451_driver(i2c, address);
        driver.reset(delay)?;
        Ok(driver)
    }

    /// Create new instance of the MMA8451 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(not(feature = "delay"))]
    pub fn new_mma8451(i2c: I2C, address: SlaveAddr) -> Result<Self, Error<E>> {
        let driver = Mma8x5x::new_mma8451_driver(i2c, address);
        driver.reset()?;
        Ok(driver)
    }

    /// Create new instance of the MMA8451 device.
    fn new_mma8451_driver(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x {
            i2c,
            xyz_data_cfg: Config::default(),
            ctrl_reg1: Config::default(),
            ctrl_reg2: Config::default(),
            ctrl_reg3: Config::default(),
            pl_cfg: Config {
                bits: BitFlags::DBCNTM,
            },
            address: address.addr(MMA845X_BASE_ADDR),
            _ic: PhantomData,
            _mode: PhantomData,
        }
    }
}

impl<E, I2C> Mma8x5x<I2C, ic::Mma8452, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the MMA8452 device.
    #[cfg(not(feature = "reset_on_initialization"))]
    pub fn new_mma8452(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x::new_mma8452_driver(i2c, address)
    }

    /// Create new instance of the MMA8452 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(feature = "delay")]
    pub fn new_mma8452<D: DelayUs<u32>>(
        i2c: I2C,
        address: SlaveAddr,
        delay: &mut D,
    ) -> Result<Self, Error<E>> {
        let mut driver = Mma8x5x::new_mma8452_driver(i2c, address);
        driver.reset(delay)?;
        Ok(driver)
    }

    /// Create new instance of the MMA8452 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(not(feature = "delay"))]
    pub fn new_mma8452(i2c: I2C, address: SlaveAddr) -> Result<Self, Error<E>> {
        let driver = Mma8x5x::new_mma8452_driver(i2c, address);
        driver.reset()?;
        Ok(driver)
    }

    /// Create new instance of the MMA8452 device.
    fn new_mma8452_driver(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x {
            i2c,
            address: address.addr(MMA845X_BASE_ADDR),
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

impl<E, I2C> Mma8x5x<I2C, ic::Mma8453, mode::Standby>
where
    I2C: i2c::WriteRead<Error = E> + i2c::Write<Error = E>,
{
    /// Create new instance of the MMA8453 device.
    #[cfg(not(feature = "reset_on_initialization"))]
    pub fn new_mma8453(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x::new_mma8453_driver(i2c, address)
    }

    /// Create new instance of the MMA8453 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(feature = "delay")]
    pub fn new_mma8453<D: DelayUs<u32>>(
        i2c: I2C,
        address: SlaveAddr,
        delay: &mut D,
    ) -> Result<Self, Error<E>> {
        let mut driver = Mma8x5x::new_mma8453_driver(i2c, address);
        driver.reset(delay)?;
        Ok(driver)
    }

    /// Create new instance of the MMA8453 device.
    #[cfg(feature = "reset_on_initialization")]
    #[cfg(not(feature = "delay"))]
    pub fn new_mma8453(i2c: I2C, address: SlaveAddr) -> Result<Self, Error<E>> {
        let driver = Mma8x5x::new_mma8453_driver(i2c, address);
        driver.reset()?;
        Ok(driver)
    }

    /// Create new instance of the MMA8453 device.
    fn new_mma8453_driver(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x {
            i2c,
            xyz_data_cfg: Config::default(),
            ctrl_reg1: Config::default(),
            ctrl_reg2: Config::default(),
            ctrl_reg3: Config::default(),
            pl_cfg: Config {
                bits: BitFlags::DBCNTM,
            },
            address: address.addr(MMA845X_BASE_ADDR),
            _ic: PhantomData,
            _mode: PhantomData,
        }
    }
}
