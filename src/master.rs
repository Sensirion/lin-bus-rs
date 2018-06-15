///! LIN bus master implementation
use driver;

pub trait Master {
    type Error;
    fn send_wakeup(&mut self) -> Result<(), Self::Error>;
    fn write_frame(&mut self, pid: u8, data: &[u8]) -> Result<(), Self::Error>;
    fn read_frame(&mut self, pid: u8, data: &mut [u8]) -> Result<(), Self::Error>;
}

impl<Driver> Master for Driver
where
    Driver: driver::Master,
{
    type Error = Driver::Error;

    fn send_wakeup(&mut self) -> Result<(), Driver::Error> {
        Driver::send_wakeup(self)
    }

    fn write_frame(&mut self, pid: u8, data: &[u8]) -> Result<(), Driver::Error> {
        self.send_header(pid)?;
        self.write(data)
    }

    fn read_frame(&mut self, pid: u8, buf: &mut [u8]) -> Result<(), Driver::Error> {
        self.send_header(pid)?;
        self.read(buf)
    }
}
