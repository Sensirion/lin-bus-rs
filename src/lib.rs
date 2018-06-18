pub mod driver;
mod master;

pub use master::Master;

#[derive(Debug)]
pub enum Error {
    Timeout,
    PhysicalBus,
    Checksum,
}

/// Calculate the LIN checksum. It is defined as "The inverted eight bit sum with carry. Eight bit
/// sum with carry is equivalent to sum all values and subtract 255 every time the sum is greater
/// or equal to 256"
pub fn checksum(pid: u8, data: &[u8]) -> u8 {
    let sum = data.iter().fold(pid as u16, |sum, v| {
        let sum = sum + *v as u16;
        if sum >= 256 {
            sum - 255
        } else {
            sum
        }
    });
    !(sum as u8)
}

/// Calculate the classic checksum. It is defined as "Checksum calculation over the data bytes only
/// is called classic checksum"
pub fn classic_checksum(data: &[u8]) -> u8 {
    checksum(0u8, data)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct CheckSumTestData<'a> {
        pid: u8,
        data: &'a [u8],
        checksum: u8,
    }
    #[test]
    fn test_checksum() {
        let test_data = [
            CheckSumTestData {
                pid: 0xDD,
                data: &[0x01],
                checksum: 0x21,
            },
            CheckSumTestData {
                pid: 0x4A,
                data: &[0x55, 0x93, 0xE5],
                checksum: 0xE6,
            },
            CheckSumTestData {
                pid: 0xBF,
                data: &[0x40, 0xFF],
                checksum: 0x00,
            },
        ];
        for d in &test_data {
            assert_eq!(d.checksum, checksum(d.pid, d.data));
        }
    }
}
