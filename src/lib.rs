#![no_std]

pub mod driver;
pub mod frame;
pub mod master;

pub use crate::frame::{checksum, classic_checksum, Frame, PID};
pub use crate::master::Master;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Timeout,
    PhysicalBus,
    Checksum,
}
