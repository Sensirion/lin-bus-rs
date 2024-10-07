# Changelog

This project follows [semantic versioning](https://semver.org/).

## [Unreleased]
 * breaking: `Pid::new` returns `Result` instead of asserting in case of an error.
   ([#37](https://github.com/Sensirion/lin-bus-rs/pull/37))

## [0.4.0] (2021-12-16)

 * changed: `PID::new`, `PID::from_id`, `PCI::new_sf` and `PCI::get_type` are
   now `const fn`s. ([#33](https://github.com/Sensirion/lin-bus-rs/pull/33))
 * changed: Use Rust 2021 edition
   ([#34](https://github.com/Sensirion/lin-bus-rs/pull/34))
 * breaking: Minimal required Rust version changed to 1.57.0

## [0.3.2] (2021-10-28)

 * added: `PID::new` to construct PID's with a known value
   ([#31](https://github.com/Sensirion/lin-bus-rs/pull/31))

## [0.3.1] (2019-08-09)

 * added: `NodeAttributes` struct and helper functions to generate diagnostic
   frames  from it. ([#23](https://github.com/Sensirion/lin-bus-rs/pull/23))
 * added: `SerialNumber` and `ProductId` definitions and decode support
   ([#24](https://github.com/Sensirion/lin-bus-rs/pull/24))
 * fixed: Bug in `PCI::get_length` where the length would be returned wrong
   ([#26](https://github.com/Sensirion/lin-bus-rs/pull/26))

## [0.3.0] (2019-07-15)

 * changed: Declare some functions as `const fn`
   ([#19](https://github.com/Sensirion/lin-bus-rs/pull/19))
 * changed: Moved `PID` and `Frame` into separat module
   ([#20](https://github.com/Sensirion/lin-bus-rs/pull/20))
 * added: Support for transport layer and diagnostic frames
   ([#20](https://github.com/Sensirion/lin-bus-rs/pull/20))

## [0.2.1] (2019-05-06)

 * fixed: Decoding of frame which uses last bit
   ([#15](https://github.com/Sensirion/lin-bus-rs/pull/15))

## [0.2.0] (2019-04-18)

 * changed: Use Rust 2018 edition syntax
   ([#13](https://github.com/Sensirion/lin-bus-rs/pull/13))
 * changed: Use classic checksum on special frames. Adds
   `PID::uses_classic_checksum` and `PID::get_id`.

## [0.1.1] (2018-07-04)

 * changed: Derive Copy, Clone, PartialEq and Eq for Error enum
   ([#11](https://github.com/Sensirion/lin-bus-rs/pull/11))

## 0.1.0 (2018-06-25)

 * First crates.io release

[Unreleased]: https://github.com/Sensirion/lin-bus-rs/compare/v0.4.0...HEAD
[0.4.0]: https://github.com/Sensirion/lin-bus-rs/compare/v0.3.2...v0.4.0
[0.3.2]: https://github.com/Sensirion/lin-bus-rs/compare/v0.3.1...v0.3.2
[0.3.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Sensirion/lin-bus-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Sensirion/lin-bus-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.1.0...v0.1.1
