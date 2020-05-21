# Rust MMA8x5x Tri-Axis Accelerometers Driver

[![crates.io](https://img.shields.io/crates/v/mma8x5x.svg)](https://crates.io/crates/mma8x5x)
[![Docs](https://docs.rs/mma8x5x/badge.svg)](https://docs.rs/mma8x5x)
[![Build Status](https://travis-ci.com/eldruin/mma8x5x-rs.svg?branch=master)](https://travis-ci.com/eldruin/mma8x5x-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mma8x5x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mma8x5x-rs?branch=master)

This is a platform agnostic Rust driver for the MMA8451, MMA8452, MMA8453, MMA8652
and MMA8653 tri-axis accelerators using the [`embedded-hal`] traits.

<!-- TODO
This driver allows you to:
-->

<!-- TODO
[Introductory blog post](TODO)
-->

## The devices

The MMA8653FC is an intelligent, low-power, three-axis, capacitive micromachined accelerometer
with 10 bits of resolution. This accelerometer is packed with embedded functions with flexible
user-programmable options, configurable to two interrupt pins. Embedded interrupt functions
enable overall power savings, by relieving the host processor from continuously polling data.
There is access to either low-pass or high-pass filtered data, which minimizes the data
analysis required for jolt detection and faster transitions. The device can be configured to
generate inertial wake-up interrupt signals from any combination of the configurable embedded
functions, enabling the MMA8653FC to monitor inertial events while remaining in a low-power
mode during periods of inactivity. 

Documentation:
- Datasheets: [MMA8653FC](https://www.nxp.com/docs/en/data-sheet/MMA8653FC.pdf)

<!--
## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
```
-->

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
