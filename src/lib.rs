//! This is a platform agnostic Rust driver for the MMA8451Q, MMA8452Q, MMA8453Q, MMA8652FC
//! and MMA8653FC tri-axis accelerators using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! <!-- TODO
//! This driver allows you to:
//! -->
//!
//! <!-- TODO
//! [Introductory blog post](TODO)
//! -->
//!
//! ## The devices
//!
//! The devices are intelligent, low-power, three-axis, capacitive micromachined accelerometers
//! with 10/12/14 bits of resolution. The accelerometers are packed with embedded functions with flexible
//! user-programmable options, configurable to interrupt pins. Embedded interrupt functions
//! enable overall power savings, by relieving the host processor from continuously polling data.
//! There is access to either low-pass or high-pass filtered data, which minimizes the data
//! analysis required for jolt detection and faster transitions. The device can be configured to
//! generate inertial wake-up interrupt signals from any combination of the configurable embedded
//! functions, enabling the devices to monitor inertial events while remaining in a low-power
//! mode during periods of inactivity.
//!
//! ### Feature comparison
//!
//! (Unavailable features are marked with "-" as this is more easily readable than Yes/No)
//!
//! | Feature                                   | MMA8451 | MMA8452 | MMA8453 | MMA8652 | MMA8653 |
//! |-------------------------------------------|---------|---------|---------|---------|---------|
//! | Resolution                                | 14-bit  | 12-bit  | 10-bit  | 12-bit  | 10-bit  |
//! | Sensitivity in 2g mode (counts/g)         | 4096    | 1024    | 256     | 1024    | 256     |
//! | 32-level FIFO                             | Yes     | -       | -       | Yes     | -       |
//! | Low power mode                            | Yes     | Yes     | Yes     | Yes     | Yes     |
//! | Auto-WAKE                                 | Yes     | Yes     | Yes     | Yes     | Yes     |
//! | Auto-SLEEP                                | Yes     | Yes     | Yes     | Yes     | Yes     |
//! | High-pass filter                          | Yes     | Yes     | Yes     | Yes     | -       |
//! | Low-pass filter                           | Yes     | Yes     | Yes     | Yes     | Yes     |
//! | Transient detection with high-pass filter | Yes     | Yes     | Yes     | Yes     | -       |
//! | Fixed orientation detection               | Yes     | Yes     | Yes     | -       | Yes     |
//! | Programmable orientation detection        | Yes     | -       | -       | Yes     | -       |
//! | Data-ready interrupt                      | Yes     | Yes     | Yes     | Yes     | Yes     |
//! | Single-tap interrupt                      | Yes     | Yes     | Yes     | Yes     | -       |
//! | Double-tap interrupt                      | Yes     | Yes     | Yes     | Yes     | -       |
//! | Directional-tap interrupt                 | Yes     | Yes     | Yes     | Yes     | -       |
//! | Freefall interrupt                        | Yes     | Yes     | Yes     | Yes     | Yes     |
//! | Motion interrupt with direction           | Yes     | Yes     | Yes     | Yes     | -       |
//! | Selectable address pin                    | Yes     | Yes     | Yes     | -       | -       |
//!
//! Documentation:
//! - Datasheets:
//!     - [MMA8451Q](https://www.nxp.com/docs/en/data-sheet/MMA8451Q.pdf)
//!     - [MMA8452Q](https://www.nxp.com/docs/en/data-sheet/MMA8452Q.pdf)
//!     - [MMA8453Q](https://www.nxp.com/docs/en/data-sheet/MMA8453Q.pdf)
//!     - [MMA8652FC](https://www.nxp.com/docs/en/data-sheet/MMA8652FC.pdf)
//!     - [MMA8653FC](https://www.nxp.com/docs/en/data-sheet/MMA8653FC.pdf)
//!
//! <!-- TODO
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### ...
//!
//! ```no_run
//! ```
//! -->
//!

#![doc(html_root_url = "https://docs.rs/mma8x5x/0.1.0")]
#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
mod types;
use crate::types::MMA845X_BASE_ADDR;
pub use crate::types::{
    ic, mode, AutoSleepDataRate, DataStatus, DebounceCounterMode, EnabledInterrupts, Error,
    FrontBackOrientation, GScale, InterruptPinConfiguration, InterruptPinPolarity, InterruptStatus,
    Measurement, ModeChangeError, OutputDataRate, PortraitLandscapeOrientation,
    PortraitLandscapeStatus, PowerMode, ReadMode, SlaveAddr, SystemMode, UnscaledMeasurement,
};
mod common;
mod conversion;
mod mma845x;
mod mma865x;
mod register_access;

/// MMA8x5x device driver
#[derive(Debug)]
pub struct Mma8x5x<I2C, IC, MODE> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    ctrl_reg1: Config,
    ctrl_reg2: Config,
    ctrl_reg3: Config,
    pl_cfg: Config,
    xyz_data_cfg: Config,
    _ic: PhantomData<IC>,
    _mode: PhantomData<MODE>,
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Config {
    bits: u8,
}

impl Config {
    fn with_high(self, mask: u8) -> Self {
        Config {
            bits: self.bits | mask,
        }
    }
    fn with_low(self, mask: u8) -> Self {
        Config {
            bits: self.bits & !mask,
        }
    }
    fn is_high(&self, mask: u8) -> bool {
        (self.bits & mask) != 0
    }
}
