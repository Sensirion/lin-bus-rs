pub mod driver;
mod master;

pub use master::Master;

#[derive(Debug)]
pub enum Error {
    Timeout,
    PhysicalBus,
    Checksum,
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
