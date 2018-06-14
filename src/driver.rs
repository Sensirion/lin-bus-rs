///! Trait for a hardware driver to implement

pub use Error;

pub trait Master {
    type Error: Into<::Error>;
    fn send_wakeup(&mut self) -> Result<(), Self::Error>;
    fn send_header(&mut self, pid: u8) -> Result<(), Self::Error>;
    fn read(&mut self, buf: &mut [u8]) -> Result<(), Self::Error>;
    fn write(&mut self, data: &[u8]) -> Result<(), Self::Error>;
}


