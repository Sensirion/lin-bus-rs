#![no_std]

pub mod driver;
pub mod master;

pub use crate::master::Master;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Error {
    Timeout,
    PhysicalBus,
    Checksum,
}

/// Protected ID which is a 6 bit ID with two parity bits
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PID(u8);

impl PID {
    /// Calculate the PID from an ID.
    /// P0 = ID0 ⊕ ID1 ⊕ ID2 ⊕ ID4
    /// P1 = ¬(ID1 ⊕ ID3 ⊕ ID4 ⊕ ID5)
    pub fn from_id(id: u8) -> PID {
        assert!(id < 64, "ID must be less than 64");
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
}
