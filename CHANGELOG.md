# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/en/1.0.0/)
and this project adheres to [Semantic Versioning](http://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

...

## [0.1.1] - 2022-09-15

### Added
- Implement `PartialEq` and `Eq` on public structs.

### Fixed
 - Fixed bit changes not being written to the driver struct in the functions `set_data_rate`, `set_wake_power_mode`, `set_sleep_power_mode` and `set_auto_sleep_data_rate`.

## [0.1.0] - 2020-08-13

Initial release to crates.io.

<!-- next-url -->
[Unreleased]: https://github.com/eldruin/mma8x5x-rs/compare/v0.1.1...HEAD
[0.1.1]: https://github.com/eldruin/mma8x5x-rs/compare/v0.1.0...v0.1.1
[0.1.0]: https://github.com/eldruin/mma8x5x-rs/releases/tag/v0.1.0
