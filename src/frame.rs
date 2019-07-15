//! LIN bus frame definitions

use bitfield::BitRange;
use byteorder::{ByteOrder, LittleEndian};
use core::mem::size_of;
use num_traits::{PrimInt, Unsigned};

/// Protected ID which is a 6 bit ID with two parity bits
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(transparent)]
pub struct PID(pub(crate) u8);

impl PID {
    /// Calculate the PID from an ID.
    /// P0 = ID0 ⊕ ID1 ⊕ ID2 ⊕ ID4
    /// P1 = ¬(ID1 ⊕ ID3 ⊕ ID4 ⊕ ID5)
    pub fn from_id(id: u8) -> PID {
        assert!(id < 64, "ID must be less than 64");
        PID::from_id_const(id)
    }

    /// Const implementation of `from_id` which doesn't validate id
    const fn from_id_const(id: u8) -> PID {
        // count parity bits and check if they are even odd
        let p0 = (id & 0b1_0111).count_ones() as u8 & 0b1;
        let p1 = ((id & 0b11_1010).count_ones() as u8 + 1) & 0b1;
        PID(id | (p0 << 6u8) | (p1 << 7u8))
    }

    /// Return the contained PID
    pub const fn get(self) -> u8 {
        self.0
    }

    /// Return the contained ID
    pub const fn get_id(self) -> u8 {
        self.0 & 0b0011_1111
    }

    /// Return if the associated frame uses the classic checksum (diagnostic IDs 60 and 61 or
    /// special use IDs 62, 63)
    pub const fn uses_classic_checksum(self) -> bool {
        self.get_id() >= 60
    }
}

/// Calculate the LIN V2.1 "enhanced" checksum. It is defined as "The inverted eight bit sum with
/// carry. Eight bit sum with carry is equivalent to sum all values and subtract 255 every time the
/// sum is greater or equal to 256"
pub fn checksum(pid: PID, data: &[u8]) -> u8 {
    let sum = data.iter().fold(u16::from(pid.0), |sum, v| {
        let sum = sum + u16::from(*v);
        if sum >= 256 {
            sum - 255
        } else {
            sum
        }
    });
    !(sum as u8)
}

/// Calculate the LIN V1.3 "classic" checksum. It is defined as "Checksum calculation over the data
/// bytes only"
pub fn classic_checksum(data: &[u8]) -> u8 {
    checksum(PID(0u8), data)
}

#[derive(Debug, Eq, PartialEq)]
pub struct Frame {
    pub(crate) pid: PID,
    pub(crate) buffer: [u8; 9],
    pub(crate) data_length: usize,
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
            (offset + length) <= self.data_length * 8,
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

/// Implements the transport layer of LIN. The units that are transported in a transport layer
/// frame are called PDU (Packet Data Unit)
pub mod transport {
    use super::{Frame, PID};

    /// NAD is the address of the slave node being addressed in a request, i.e. only slave nodes
    /// have an address. NAD is also used to indicate the source of a response.
    #[repr(transparent)]
    pub struct NAD(pub u8);

    /// The PCI (Protocol Control Information) contains the transport layer flow control
    /// information.
    #[repr(transparent)]
    pub struct PCI(u8);

    /// Type of the `PCI` byte
    pub enum PCIType {
        /// Single Frame
        SF = 0,
        /// First Frame. Start of a multi frame message.
        FF = 1,
        /// Consecutive Frame.
        CF = 2,
        /// Invalid PCIType
        Invalid,
    }

    impl PCI {
        pub fn new_sf(length: u8) -> PCI {
            assert!(length <= 5, "Maximum length for single frame is 5");
            PCI(length + 1)
        }

        pub fn get_type(self) -> PCIType {
            match self.0 >> 4 {
                0 => PCIType::SF,
                1 => PCIType::FF,
                2 => PCIType::CF,
                _ => PCIType::Invalid,
            }
        }

        pub const fn get_length(self) -> u8 {
            self.0
        }
    }

    /// The Service Identifier (SID) specifies the request that shall be performed by the slave
    /// node addressed.
    #[repr(transparent)]
    pub struct SID(pub u8);

    #[repr(transparent)]
    /// The Response Service Identifier (RSID) specifies the contents of the response.
    pub struct RSID(pub u8);

    /// Create a single frame (CF) PDU
    pub fn create_single_frame(pid: PID, nad: NAD, sid: SID, data: &[u8]) -> Frame {
        assert!(
            !data.is_empty() && data.len() <= 5,
            "A single frame must contain between 0 and 5 bytes"
        );
        // If a PDU is not completely filled the unused bytes shall be filled with 0xFF.
        let mut frame_data = [0xFFu8; 8];
        frame_data[0] = nad.0;
        frame_data[1] = PCI::new_sf(data.len() as u8).0;
        frame_data[2] = sid.0;
        frame_data[3..data.len() + 3].clone_from_slice(data);
        Frame::from_data(pid, &frame_data)
    }
}

/// Implements the LIN diagnostics methods.
pub mod diagnostic {
    use super::PID;

    pub const MASTER_REQUEST_FRAME_ID: u8 = 0x3C;
    pub const SLAVE_RESPONSE_FRAME_ID: u8 = 0x3D;

    pub const MASTER_REQUEST_FRAME_PID: PID = PID::from_id_const(0x3C);
    pub const SLAVE_RESPONSE_FRAME_PID: PID = PID::from_id_const(0x3D);
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CheckSumTestData<'a> {
        pid: PID,
        data: &'a [u8],
        checksum: u8,
    }

    #[test]
    fn test_enhanced_checksum() {
        let test_data = [
            CheckSumTestData {
                pid: PID(0xDD),
                data: &[0x01],
                checksum: 0x21,
            },
            CheckSumTestData {
                pid: PID(0x4A),
                data: &[0x55, 0x93, 0xE5],
                checksum: 0xE6,
            },
            CheckSumTestData {
                pid: PID(0xBF),
                data: &[0x40, 0xFF],
                checksum: 0x00,
            },
        ];
        for d in &test_data {
            assert_eq!(d.checksum, checksum(d.pid, d.data));
        }
    }

    #[test]
    fn test_classic_checksum() {
        let test_data = [
            CheckSumTestData {
                pid: PID::from_id(0x3C),
                data: &[0x01],
                checksum: 0xFE,
            },
            CheckSumTestData {
                pid: PID::from_id(0x3D),
                data: &[0x01],
                checksum: 0xFE,
            },
            CheckSumTestData {
                pid: PID::from_id(0x3d),
                data: &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08],
                checksum: 0xDB,
            },
        ];
        for d in &test_data {
            assert_eq!(d.checksum, classic_checksum(d.data));
        }
    }

    #[test]
    fn test_pid_from_id() {
        let test_data = [
            (0, PID(0x80)),
            (1, PID(0xC1)),
            (2, PID(0x42)),
            (25, PID(0x99)),
            (27, PID(0x5B)),
            (29, PID(0xDD)),
        ];

        for d in &test_data {
            let pid = PID::from_id(d.0);
            assert_eq!(pid, d.1);
            assert_eq!(pid.get_id(), d.0);
        }
    }

    #[test]
    fn test_id_uses_classic_checksum() {
        let test_ids: &[u8] = &[0, 1, 59, 60, 63];

        for i in test_ids {
            assert_eq!(PID::from_id(*i).uses_classic_checksum(), *i >= 60);
        }
    }

    #[test]
    #[should_panic]
    fn test_pid_from_id_panic() {
        PID::from_id(64);
    }

    #[test]
    fn test_transport_frame() {
        const LIN_ID_SERIAL_REQ_PAYLOAD: &[u8] = &[0x10, 0x06, 0xB2, 0x01, 0xB3, 0x00, 0x01, 0x10];
        let frame = transport::create_single_frame(
            diagnostic::MASTER_REQUEST_FRAME_PID,
            transport::NAD(0x10),
            transport::SID(0xB2),
            &[0x01, 0xB3, 0x00, 0x01, 0x10],
        );

        assert_eq!(frame.get_pid(), diagnostic::MASTER_REQUEST_FRAME_PID);
        assert_eq!(frame.get_data(), LIN_ID_SERIAL_REQ_PAYLOAD);
        assert_eq!(frame.data_length, 8);
    }
}
