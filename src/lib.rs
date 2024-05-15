//! This is a platform agnostic Rust driver for the MMA8451Q, MMA8452Q, MMA8453Q, MMA8652FC
//! and MMA8653FC tri-axis accelerators using the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Change mode to active/standby. See: [`into_active()`].
//! - Read raw unscaled measurement. See: [`read_unscaled()`].
//! - Read measurement. See: [`read()`].
//! - Read data status. See: [`data_status()`].
//! - Read system operating mode. See: [`system_mode()`].
//! - Set G scale. See: [`set_scale()`].
//! - Set data rate. See [`set_data_rate()`].
//! - Set wake power mode. See [`set_wake_power_mode()`].
//! - Set sleep power mode. See [`set_sleep_power_mode()`].
//! - Set read mode. See: [`set_read_mode()`].
//! - Set offset correction. See: [`set_offset_correction()`].
//! - Read the device ID. See: [`device_id()`].
//! - Reset device. See: [`reset()`].
//! - Enable/disable self-test mode. See: [`enable_self_test()`].
//! - Auto-sleep/wake:
//!     - Enable/disable auto-sleep/wake. See: [`enable_auto_sleep()`].
//!     - Set auto-sleep data rate. See: [`set_auto_sleep_data_rate()`].
//!     - Set auto-sleep count. See: [`set_auto_sleep_count()`].
//! - Portrait/Landscape detection:
//!     - Enable/disable portrait/landscape detection. See: [`enable_portrait_landscape_detection()`].
//!     - Set debounce counter mode. See: [`set_debounce_counter_mode()`].
//!     - Set debounce counter. See: [`set_debounce_counter()`].
//!     - Read portrait/landscape status. See: [`portrait_landscape_status()`].
//! - Interrupts:
//!     - Enable/disable interrupts. See: [`set_enabled_interrupts()`].
//!     - Set interrupt pin routes. See: [`set_interrupt_pin_routes()`].
//!     - Set interrupt pin polarity. See: [`set_interrupt_pin_polarity()`].
//!     - Set interrupt pin configuration. See: [`set_interrupt_pin_configuration()`].
//!     - Set interrupts that wake the device from sleep. See: [`set_wake_interrupts()`].
//!     - Read interrupt status. See: [`interrupt_status()`].
//!
//! [`into_active()`]: struct.Mma8x5x.html#method.into_active
//! [`read_unscaled()`]: struct.Mma8x5x.html#method.read_unscaled
//! [`read()`]: struct.Mma8x5x.html#method.read
//! [`data_status()`]: struct.Mma8x5x.html#method.data_status
//! [`system_mode()`]: struct.Mma8x5x.html#method.system_mode
//! [`set_scale()`]: struct.Mma8x5x.html#method.set_scale
//! [`set_data_rate()`]: struct.Mma8x5x.html#method.set_data_rate
//! [`set_wake_power_mode()`]: struct.Mma8x5x.html#method.set_wake_power_mode
//! [`set_sleep_power_mode()`]: struct.Mma8x5x.html#method.set_sleep_power_mode
//! [`set_read_mode()`]: struct.Mma8x5x.html#method.set_read_mode
//! [`set_offset_correction()`]: struct.Mma8x5x.html#method.set_offset_correction
//! [`device_id()`]: struct.Mma8x5x.html#method.device_id
//! [`reset()`]: struct.Mma8x5x.html#method.reset
//! [`enable_self_test()`]: struct.Mma8x5x.html#method.enable_self_test
//! [`enable_auto_sleep()`]: struct.Mma8x5x.html#method.enable_auto_sleep
//! [`set_auto_sleep_data_rate()`]: struct.Mma8x5x.html#method.set_auto_sleep_data_rate
//! [`set_auto_sleep_count()`]: struct.Mma8x5x.html#method.set_auto_sleep_count
//! [`enable_portrait_landscape_detection()`]: struct.Mma8x5x.html#method.enable_portrait_landscape_detection
//! [`set_debounce_counter_mode()`]: struct.Mma8x5x.html#method.set_debounce_counter_mode
//! [`set_debounce_counter()`]: struct.Mma8x5x.html#method.set_debounce_counter
//! [`portrait_landscape_status()`]: struct.Mma8x5x.html#method.portrait_landscape_status
//! [`set_enabled_interrupts()`]: struct.Mma8x5x.html#method.set_enabled_interrupts
//! [`set_interrupt_pin_routes()`]: struct.Mma8x5x.html#method.set_interrupt_pin_routes
//! [`set_interrupt_pin_polarity()`]: struct.Mma8x5x.html#method.set_interrupt_pin_polarity
//! [`set_interrupt_pin_configuration()`]: struct.Mma8x5x.html#method.set_interrupt_pin_configuration
//! [`set_wake_interrupts()`]: struct.Mma8x5x.html#method.set_wake_interrupts
//! [`interrupt_status()`]: struct.Mma8x5x.html#method.interrupt_status
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
//! (Unavailable features are marked with "-" as this is more easily readable than Yes/No)
//!
//! Documentation:
//! - Datasheets:
//!     - [MMA8451Q](https://www.nxp.com/docs/en/data-sheet/MMA8451Q.pdf)
//!     - [MMA8452Q](https://www.nxp.com/docs/en/data-sheet/MMA8452Q.pdf)
//!     - [MMA8453Q](https://www.nxp.com/docs/en/data-sheet/MMA8453Q.pdf)
//!     - [MMA8652FC](https://www.nxp.com/docs/en/data-sheet/MMA8652FC.pdf)
//!     - [MMA8653FC](https://www.nxp.com/docs/en/data-sheet/MMA8653FC.pdf)
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Most of the settings can only be changed while the device is in standby mode.
//! Then the mode can be changed to active and acceleration measurements read.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Change mode to active and read acceleration
//!
//! Using an MMA8653
//!
//! ```no_run
//! use linux_embedded_hal::I2cdev;
//! use mma8x5x::Mma8x5x;
//!
//! let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let sensor = Mma8x5x::new_mma8653(dev);
//! let mut sensor = sensor.into_active().ok().unwrap();
//! loop {
//!     let accel = sensor.read().unwrap();
//!     println!("Acceleration: {:?}", accel);
//! }
//! ```
//!
//! ### Change mode to active and read raw unscaled acceleration
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! use mma8x5x::{Mma8x5x, SlaveAddr};
//!
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let sensor = Mma8x5x::new_mma8452(dev, SlaveAddr::default());
//! let mut sensor = sensor.into_active().ok().unwrap();
//! loop {
//!     let accel = sensor.read_unscaled().unwrap();
//!     println!("Raw acceleration: {:?}", accel);
//! }
//! ```
//!
//! ### Use alternative address
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! use mma8x5x::{Mma8x5x, SlaveAddr};
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let sensor = Mma8x5x::new_mma8451(dev, SlaveAddr::Alternative(true));
//! ```
//!
//! ### Set scale to +/-8g and 200Hz ODR, then read acceleration
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! use mma8x5x::{Mma8x5x, GScale, OutputDataRate};
//!
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Mma8x5x::new_mma8652(dev);
//! sensor.set_scale(GScale::G8).unwrap();
//! sensor.set_data_rate(OutputDataRate::Hz200).unwrap();
//! let mut sensor = sensor.into_active().ok().unwrap();
//! loop {
//!     let accel = sensor.read().unwrap();
//!     println!("Acceleration: {:?}", accel);
//! }
//! ```
//!
//! ### Configure auto-sleep/wake mode
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! use mma8x5x::{Mma8x5x, AutoSleepDataRate, EnabledInterrupts, PowerMode};
//!
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Mma8x5x::new_mma8652(dev);
//! sensor.set_auto_sleep_data_rate(AutoSleepDataRate::Hz12_5).unwrap();
//! sensor.set_auto_sleep_count(125).unwrap();
//! sensor.set_sleep_power_mode(PowerMode::LowPower).unwrap();
//! sensor.enable_auto_sleep().unwrap();
//! // ...
//! let mut sensor = sensor.into_active().ok().unwrap();
//! ```
//!
//! ### Enable portrait/landscape detection and interrupt generation
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! use mma8x5x::{Mma8x5x, EnabledInterrupts, InterruptPinPolarity};
//!
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Mma8x5x::new_mma8652(dev);
//! sensor.set_enabled_interrupts(EnabledInterrupts {
//!     portrait_landscape: true,
//!     ..EnabledInterrupts::default() // the rest stays disabled
//! }).unwrap();
//! sensor.enable_portrait_landscape_detection().unwrap();
//! sensor.set_interrupt_pin_polarity(InterruptPinPolarity::ActiveHigh).unwrap();
//! let mut sensor = sensor.into_active().ok().unwrap();
//! loop {
//!     let pl_status = sensor.portrait_landscape_status();
//!     println!("P/L status: {:?}", pl_status);
//!     let int_status = sensor.interrupt_status();
//!     println!("Interrupt status: {:?}", int_status);
//! }
//! ```
//!
//! ### Enable self-test
//!
//! ```no_run
//! # use linux_embedded_hal::I2cdev;
//! # use mma8x5x::Mma8x5x;
//! #
//! # let dev = I2cdev::new("/dev/i2c-1").unwrap();
//! let mut sensor = Mma8x5x::new_mma8652(dev);
//! sensor.enable_self_test().unwrap();
//! let mut sensor = sensor.into_active().ok().unwrap();
//! loop {
//!     let accel = sensor.read().unwrap();
//!     println!("Acceleration: {:?}", accel);
//! }
//! ```

#![deny(unsafe_code, missing_docs)]
#![no_std]

use core::marker::PhantomData;
mod types;
use crate::types::MMA845X_BASE_ADDR;
pub use crate::types::{
    ic, mode, AutoSleepDataRate, DataStatus, DebounceCounterMode, EnabledInterrupts, Error,
    FreefallMotionConfiguration, FreefallMotionDebounceMode, FreefallMotionDetectionMode,
    FrontBackOrientation, GScale, InterruptPinConfiguration, InterruptPinPolarity,
    InterruptPinRoutes, InterruptSourcePinRoute, InterruptStatus, Measurement, ModeChangeError,
    OutputDataRate, PortraitLandscapeOrientation, PortraitLandscapeStatus, PowerMode, ReadMode,
    SlaveAddr, SystemMode, UnscaledMeasurement, WakeInterrupts,
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
