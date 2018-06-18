///! LIN bus master implementation
use checksum;
use driver;

pub trait Master {
    type Error;
    fn send_wakeup(&mut self) -> Result<(), Self::Error>;
    fn write_frame(&mut self, pid: u8, data: &[u8]) -> Result<(), Self::Error>;
    fn read_frame(&mut self, pid: u8, data: &mut [u8]) -> Result<(), Self::Error>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    pid: u8,
    buffer: [u8; 9],
    data_length: usize,
}

impl Frame {
    /// Creates a LIN frame from the PID and data. Calculates and adds checksum accordingly
    pub fn from_data(pid: u8, data: &[u8]) -> Frame {
        assert!(data.len() <= 8, "Maximum data is 8 bytes");
        let mut buffer = [0u8; 9];
        buffer[0..data.len()].clone_from_slice(data);
        buffer[data.len()] = checksum(pid, &buffer[0..data.len()]);
        Frame {
            pid: pid,
            buffer: buffer,
            data_length: data.len(),
        }
    }

    /// Access the data from the frame
    pub fn get_data(&self) -> &[u8] {
        &self.buffer[0..self.data_length]
    }

    /// Get the checksum from the frame
    pub fn get_checksum(&self) -> u8 {
        self.buffer[self.data_length]
    }

    /// Get the PID from the frame
    pub fn get_pid(&self) -> u8 {
        self.pid
    }

    /// Get the serialized bytes to write to the driver
    pub fn get_data_with_checksum(&self) -> &[u8] {
        &self.buffer[0..=self.data_length]
    }
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

#[cfg(test)]
mod tests {
    use super::*;

    struct FrameTestData<'a> {
        pid: u8,
        data: &'a [u8],
        frame: Frame,
    }
    #[test]
    fn test_frame_from_data() {
        let test_data = [FrameTestData {
            pid: 0xDD,
            data: &[0x01],
            frame: Frame {
                pid: 0xDD,
                buffer: [0x01, 0x21, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00],
                data_length: 1,
            },
        }];
        for d in &test_data {
            let frame = Frame::from_data(d.pid, d.data);
            assert_eq!(frame, d.frame);
            assert_eq!(frame.get_data(), d.data);
            assert_eq!(frame.get_pid(), d.pid);
            assert_eq!(frame.get_data_with_checksum().len(), d.data.len() + 1);
        }
    }
}
