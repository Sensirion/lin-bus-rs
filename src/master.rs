//! LIN bus master implementation
use crate::driver;
use crate::frame::Frame;
use crate::PID;
use crate::{checksum, classic_checksum};

pub trait Master {
    type Error;
    fn send_wakeup(&mut self) -> Result<(), Self::Error>;
    fn write_frame(&mut self, frame: &Frame) -> Result<(), Self::Error>;
    fn read_frame(&mut self, pid: PID, data_lengh: usize) -> Result<Frame, Self::Error>;
}

impl<Driver> Master for Driver
where
    Driver: driver::Master,
{
    type Error = Driver::Error;

    fn send_wakeup(&mut self) -> Result<(), Driver::Error> {
        Driver::send_wakeup(self)
    }

    fn write_frame(&mut self, frame: &Frame) -> Result<(), Driver::Error> {
        self.send_header(frame.get_pid())?;
        self.write(frame.get_data_with_checksum())
    }

    fn read_frame(&mut self, pid: PID, data_length: usize) -> Result<Frame, Driver::Error> {
        assert!(data_length <= 8, "Maximum data length is 8 bytes");
        self.send_header(pid)?;
        let mut frame = Frame {
            pid,
            data_length,
            buffer: [0u8; 9],
        };
        self.read(&mut frame.buffer[0..=data_length])?;

        let checksum = {
            if pid.uses_classic_checksum() {
                classic_checksum(&frame.buffer[0..data_length])
            } else {
                checksum(pid, &frame.buffer[0..data_length])
            }
        };
        if checksum != frame.buffer[data_length] {
            Err(Driver::Error::from(driver::Error::Checksum))
        } else {
            Ok(frame)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct FrameTestData<'a> {
        pid: PID,
        data: &'a [u8],
        frame: Frame,
    }
    #[test]
    fn test_frame_from_data() {
        let test_data = [FrameTestData {
            pid: PID::new(0xDD).unwrap(),
            data: &[0x01],
            frame: Frame {
                pid: PID::new(0xDD).unwrap(),
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

    #[test]
    fn test_data_decode() {
        let test_data = [
            (
                Frame::from_data(PID::new(80).unwrap(), &[254, 251, 239, 255]),
                [1022, 1022, 2046],
            ),
            (
                Frame::from_data(PID::new(80).unwrap(), &[3, 12, 240, 182]),
                [3, 3, 879],
            ),
            (
                Frame::from_data(PID::new(80).unwrap(), &[3, 12, 0, 183]),
                [3, 3, 880],
            ),
            (
                Frame::from_data(PID::new(80).unwrap(), &[2, 12, 240, 182]),
                [2, 3, 879],
            ),
            (
                Frame::from_data(PID::new(80).unwrap(), &[2, 8, 0, 183]),
                [2, 2, 880],
            ),
        ];

        for d in &test_data {
            assert_eq!(d.0.decode::<u16>(0, 10), d.1[0]);
            assert_eq!(d.0.decode::<u16>(10, 10), d.1[1]);
            assert_eq!(d.0.decode::<u16>(20, 11), d.1[2]);
        }
    }

    #[test]
    fn test_data_decode_all_bits() {
        let frame = Frame::from_data(PID::new(80).unwrap(), &[0x55, 0xDD]);
        assert_eq!(frame.decode::<u16>(0, 16), 0xdd55);
    }
}
