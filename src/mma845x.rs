use crate::{ic, Mma8x5x, SlaveAddr, MMA845X_BASE_ADDR};
use core::marker::PhantomData;

impl<I2C> Mma8x5x<I2C, ic::Mma8451> {
    /// Create new instance of the MMA8451 device.
    pub fn new_mma8451(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x {
            i2c,
            address: address.addr(MMA845X_BASE_ADDR),
            _ic: PhantomData,
        }
    }
}

impl<I2C> Mma8x5x<I2C, ic::Mma8452> {
    /// Create new instance of the MMA8452 device.
    pub fn new_mma8452(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x {
            i2c,
            address: address.addr(MMA845X_BASE_ADDR),
            _ic: PhantomData,
        }
    }
}

impl<I2C> Mma8x5x<I2C, ic::Mma8453> {
    /// Create new instance of the MMA8453 device.
    pub fn new_mma8453(i2c: I2C, address: SlaveAddr) -> Self {
        Mma8x5x {
            i2c,
            address: address.addr(MMA845X_BASE_ADDR),
            _ic: PhantomData,
        }
    }
}
