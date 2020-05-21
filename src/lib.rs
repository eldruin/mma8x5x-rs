//! This is a platform agnostic Rust driver for the MMA8450, MMA8451, MMA8452, MMA8453, MMA8652
//! and MMA8653 tri-axis accelerators using the [`embedded-hal`] traits.
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
//! The MMA8653FC is an intelligent, low-power, three-axis, capacitive micromachined accelerometer
//! with 10 bits of resolution. This accelerometer is packed with embedded functions with flexible
//! user-programmable options, configurable to two interrupt pins. Embedded interrupt functions
//! enable overall power savings, by relieving the host processor from continuously polling data.
//! There is access to either low-pass or high-pass filtered data, which minimizes the data
//! analysis required for jolt detection and faster transitions. The device can be configured to
//! generate inertial wake-up interrupt signals from any combination of the configurable embedded
//! functions, enabling the MMA8653FC to monitor inertial events while remaining in a low-power
//! mode during periods of inactivity.
//!
//! Documentation:
//! - Datasheets: [MMA8653FC](https://www.nxp.com/docs/en/data-sheet/MMA8653FC.pdf)
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
pub use crate::types::{ic, Error};
mod common;
mod mma8653;
mod register_access;

/// MMA8x5x device driver
#[derive(Debug)]
pub struct Mma8x5x<I2C, IC> {
    /// The concrete I²C device implementation.
    i2c: I2C,
    address: u8,
    _ic: PhantomData<IC>,
}
