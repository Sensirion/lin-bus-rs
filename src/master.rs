///! LIN bus master implementation
use crate::driver;
use crate::PID;
use crate::{checksum, classic_checksum};
use bitfield::BitRange;
use byteorder::{ByteOrder, LittleEndian};
use core::mem::size_of;
use num_traits::{PrimInt, Unsigned};

pub trait Master {
    type Error;
    fn send_wakeup(&mut self) -> Result<(), Self::Error>;
    fn write_frame(&mut self, frame: &Frame) -> Result<(), Self::Error>;
    fn read_frame(&mut self, pid: PID, data_lengh: usize) -> Result<Frame, Self::Error>;
}

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    pid: PID,
    buffer: [u8; 9],
    data_length: usize,
}

impl Frame {
    /// Creates a LIN frame from the PID and data. Calculates and adds checksum accordingly
    pub fn from_data(pid: PID, data: &[u8]) -> Frame {
        assert!(data.len() <= 8, "Maximum data is 8 bytes");
        let mut buffer = [0u8; 9];
        buffer[0..data.len()].clone_from_slice(data);
        buffer[data.len()] = {
            if pid.uses_classic_checksum() {
                classic_checksum(&buffer[0..data.len()])
            } else {
                checksum(pid, &buffer[0..data.len()])
            }
        };
        Frame {
            pid,
            buffer,
            data_length: data.len(),
        }
    }

    /// Access the data from the frame
    pub fn get_data(&self) -> &[u8] {
        &self.buffer[0..self.data_length]
    }

    /// Decode frame data
    pub fn decode<T>(&self, offset: usize, length: usize) -> T
    where
        T: PrimInt + Unsigned,
        u64: BitRange<T>,
    {
        assert!(
            (offset + length) < self.data_length * 8,
            "Not enough data available"
        );
        assert!(length <= size_of::<T>() * 8, "Output type not big enough");

        let num = LittleEndian::read_u64(&self.buffer[0..8]);
        num.bit_range(offset + length - 1, offset)
    }

    /// Get the checksum from the frame
    pub fn get_checksum(&self) -> u8 {
        self.buffer[self.data_length]
    }

    /// Get the PID from the frame
    pub fn get_pid(&self) -> PID {
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

    fn write_frame(&mut self, frame: &Frame) -> Result<(), Driver::Error> {
        self.send_header(frame.get_pid())?;
        self.write(frame.get_data_with_checksum())
    }

    fn read_frame(&mut self, pid: PID, data_length: usize) -> Result<Frame, Driver::Error> {
        assert!(data_length <= 8, "Maximum data length is 8 bytes");
        self.send_header(pid)?;
        let mut frame = Frame {
            pid: pid,
            data_length: data_length,
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
            pid: PID(0xDD),
            data: &[0x01],
            frame: Frame {
                pid: PID(0xDD),
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
                Frame::from_data(PID(0), &[254, 251, 239, 255]),
                [1022, 1022, 2046],
            ),
            (Frame::from_data(PID(0), &[3, 12, 240, 182]), [3, 3, 879]),
            (Frame::from_data(PID(0), &[3, 12, 0, 183]), [3, 3, 880]),
            (Frame::from_data(PID(0), &[2, 12, 240, 182]), [2, 3, 879]),
            (Frame::from_data(PID(0), &[2, 8, 0, 183]), [2, 2, 880]),
        ];

        for d in &test_data {
            assert_eq!(d.0.decode::<u16>(0, 10), d.1[0]);
            assert_eq!(d.0.decode::<u16>(10, 10), d.1[1]);
            assert_eq!(d.0.decode::<u16>(20, 11), d.1[2]);
        }
    }
}
