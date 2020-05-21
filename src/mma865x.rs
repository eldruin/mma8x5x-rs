//! MLX90614-specific functions

use crate::{ic, Mma8x5x};
use core::marker::PhantomData;

impl<I2C> Mma8x5x<I2C, ic::Mma8652> {
    /// Create new instance of the MMA8652 device.
    pub fn new_mma8652(i2c: I2C) -> Self {
        Mma8x5x {
            i2c,
            address: 0x1D,
            _ic: PhantomData,
        }
    }
}

impl<I2C> Mma8x5x<I2C, ic::Mma8653> {
    /// Create new instance of the MMA8653 device.
    pub fn new_mma8653(i2c: I2C) -> Self {
        Mma8x5x {
            i2c,
            address: 0x1D,
            _ic: PhantomData,
        }
    }
}
