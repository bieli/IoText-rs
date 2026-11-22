pub struct Tools;

pub const CRC16_POLY_DEFAULT: u32 = 0xA001;

impl Tools {
    /// CRC-16 (reflected, init `0xFFFF`) over the UTF-8 bytes of `data`,
    /// formatted as four upper-case hex digits.
    #[must_use]
    pub fn crc16(data: &str, poly: u32) -> String {
        let mut crc: u32 = 0xFFFF;
        for byte in data.bytes() {
            crc ^= u32::from(byte);
            for _ in 0..8 {
                crc = if (crc & 0x0001) != 0 {
                    (crc >> 1) ^ poly
                } else {
                    crc >> 1
                };
            }
        }
        format!("{crc:04X}")
    }
}
