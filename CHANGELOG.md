# Changelog

This project follows [semantic versioning](https://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/Sensirion/lin-bus-rs/compare/v0.3.1...HEAD
[0.3.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.3.0...v0.3.1
[0.3.0]: https://github.com/Sensirion/lin-bus-rs/compare/v0.2.1...v0.3.0
[0.2.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Sensirion/lin-bus-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.1.0...v0.1.1
