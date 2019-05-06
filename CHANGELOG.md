# Changelog

This project follows [semantic versioning](https://semver.org/).

## [Unreleased]

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

[Unreleased]: https://github.com/Sensirion/lin-bus-rs/compare/v0.2.1...HEAD
[0.2.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.2.0...v0.2.1
[0.2.0]: https://github.com/Sensirion/lin-bus-rs/compare/v0.1.1...v0.2.0
[0.1.1]: https://github.com/Sensirion/lin-bus-rs/compare/v0.1.0...v0.1.1
