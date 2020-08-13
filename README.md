# Rust MMA8x5x Tri-Axis Accelerometers Driver

[![crates.io](https://img.shields.io/crates/v/mma8x5x.svg)](https://crates.io/crates/mma8x5x)
[![Docs](https://docs.rs/mma8x5x/badge.svg)](https://docs.rs/mma8x5x)
[![Build Status](https://travis-ci.com/eldruin/mma8x5x-rs.svg?branch=master)](https://travis-ci.com/eldruin/mma8x5x-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mma8x5x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mma8x5x-rs?branch=master)

This is a platform agnostic Rust driver for the MMA8451Q, MMA8452Q, MMA8453Q, MMA8652FC
and MMA8653FC tri-axis accelerators using the [`embedded-hal`] traits.

This driver allows you to:
- Change mode to active/standby. See: `into_active()`.
- Read raw unscaled measurement. See: `read_unscaled()`.
- Read measurement. See: `read()`.
- Read data status. See: `data_status()`.
- Read system operating mode. See: `system_mode()`.
- Set G scale. See: `set_scale()`.
- Set data rate. See `set_data_rate()`.
- Set wake power mode. See `set_wake_power_mode()`.
- Set sleep power mode. See `set_sleep_power_mode()`.
- Set read mode. See: `set_read_mode()`.
- Set offset correction. See: `set_offset_correction()`.
- Read the device ID. See: `device_id()`.
- Reset device. See: `reset()`.
- Enable/disable self-test mode. See: `enable_self_test()`.
- Auto-sleep/wake:
    - Enable/disable auto-sleep/wake. See: `enable_auto_sleep()`.
    - Set auto-sleep data rate. See: `set_auto_sleep_data_rate()`.
    - Set auto-sleep count. See: `set_auto_sleep_count()`.
- Portrait/Landscape detection:
    - Enable/disable portrait/landscape detection. See: `enable_portrait_landscape_detection()`.
    - Set debounce counter mode. See: `set_debounce_counter_mode()`.
    - Set debounce counter. See: `set_debounce_counter()`.
    - Read portrait/landscape status. See: `portrait_landscape_status()`.
- Interrupts:
    - Enable/disable interrupts. See: `set_enabled_interrupts()`.
    - Set interrupt pin routes. See: `set_interrupt_pin_routes()`.
    - Set interrupt pin polarity. See: `set_interrupt_pin_polarity()`.
    - Set interrupt pin configuration. See: `set_interrupt_pin_configuration()`.
    - Set interrupts that wake the device from sleep. See: `set_wake_interrupts()`.
    - Read interrupt status. See: `interrupt_status()`.

<!-- TODO
[Introductory blog post](TODO)
-->

## The devices

The devices are intelligent, low-power, three-axis, capacitive micromachined accelerometers
with 10/12/14 bits of resolution. The accelerometers are packed with embedded functions with flexible
user-programmable options, configurable to interrupt pins. Embedded interrupt functions
enable overall power savings, by relieving the host processor from continuously polling data.
There is access to either low-pass or high-pass filtered data, which minimizes the data
analysis required for jolt detection and faster transitions. The device can be configured to
generate inertial wake-up interrupt signals from any combination of the configurable embedded
functions, enabling the devices to monitor inertial events while remaining in a low-power
mode during periods of inactivity.

### Feature comparison

| Feature                                   | MMA8451 | MMA8452 | MMA8453 | MMA8652 | MMA8653 |
|-------------------------------------------|---------|---------|---------|---------|---------|
| Resolution                                | 14-bit  | 12-bit  | 10-bit  | 12-bit  | 10-bit  |
| Sensitivity in 2g mode (counts/g)         | 4096    | 1024    | 256     | 1024    | 256     |
| 32-level FIFO                             | Yes     | -       | -       | Yes     | -       |
| Low power mode                            | Yes     | Yes     | Yes     | Yes     | Yes     |
| Auto-WAKE                                 | Yes     | Yes     | Yes     | Yes     | Yes     |
| Auto-SLEEP                                | Yes     | Yes     | Yes     | Yes     | Yes     |
| High-pass filter                          | Yes     | Yes     | Yes     | Yes     | -       |
| Low-pass filter                           | Yes     | Yes     | Yes     | Yes     | Yes     |
| Transient detection with high-pass filter | Yes     | Yes     | Yes     | Yes     | -       |
| Fixed orientation detection               | Yes     | Yes     | Yes     | -       | Yes     |
| Programmable orientation detection        | Yes     | -       | -       | Yes     | -       |
| Data-ready interrupt                      | Yes     | Yes     | Yes     | Yes     | Yes     |
| Single-tap interrupt                      | Yes     | Yes     | Yes     | Yes     | -       |
| Double-tap interrupt                      | Yes     | Yes     | Yes     | Yes     | -       |
| Directional-tap interrupt                 | Yes     | Yes     | Yes     | Yes     | -       |
| Freefall interrupt                        | Yes     | Yes     | Yes     | Yes     | Yes     |
| Motion interrupt with direction           | Yes     | Yes     | Yes     | Yes     | -       |
| Selectable address pin                    | Yes     | Yes     | Yes     | -       | -       |

(Unavailable features are marked with "-" as this is more easily readable than Yes/No)

Documentation:
- Datasheets:
    - [MMA8451Q](https://www.nxp.com/docs/en/data-sheet/MMA8451Q.pdf)
    - [MMA8452Q](https://www.nxp.com/docs/en/data-sheet/MMA8452Q.pdf)
    - [MMA8453Q](https://www.nxp.com/docs/en/data-sheet/MMA8453Q.pdf)
    - [MMA8652FC](https://www.nxp.com/docs/en/data-sheet/MMA8652FC.pdf)
    - [MMA8653FC](https://www.nxp.com/docs/en/data-sheet/MMA8653FC.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

Most of the settings can only be changed while the device is in standby mode.
Then the mode can be changed to active and acceleration measurements read.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use linux_embedded_hal::I2cdev;
use mma8x5x::Mma8x5x;

fn main() {
    let dev = I2cdev::new("/dev/i2c-1").unwrap();
    let sensor = Mma8x5x::new_mma8653(dev);
    let mut sensor = sensor.into_active().ok().unwrap();
    loop {
        let accel = sensor.read().unwrap();
        println!("Acceleration: {:?}", accel);
    }
}
```

## Support

For questions, issues, feature requests, and other changes, please file an
[issue in the github project](https://github.com/eldruin/mma8x5x-rs/issues).

## License

Licensed under either of:

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
