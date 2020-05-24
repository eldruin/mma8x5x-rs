use crate::{ic, mode, register_access::BitFlags, Config, Mma8x5x, SlaveAddr, MMA845X_BASE_ADDR};
use core::marker::PhantomData;

impl<I2C> Mma8x5x<I2C, ic::Mma8451, mode::Standby> {
    /// Create new instance of the MMA8451 device.
    pub fn new_mma8451(i2c: I2C, address: SlaveAddr) -> Self {
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

impl<I2C> Mma8x5x<I2C, ic::Mma8452, mode::Standby> {
    /// Create new instance of the MMA8452 device.
    pub fn new_mma8452(i2c: I2C, address: SlaveAddr) -> Self {
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

impl<I2C> Mma8x5x<I2C, ic::Mma8453, mode::Standby> {
    /// Create new instance of the MMA8453 device.
    pub fn new_mma8453(i2c: I2C, address: SlaveAddr) -> Self {
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
