//! CRC-16 used by the `c|XXXX` item (reflected, polynomial `0xA001`, init `0xFFFF`,
//! a.k.a. CRC-16/MODBUS).

use alloc::format;
use alloc::string::String;

/// Reflected CRC-16 polynomial used by the protocol.
pub const CRC16_POLY_DEFAULT: u32 = 0xA001;

/// Bitwise CRC-16 for an arbitrary reflected polynomial (reference implementation).
#[must_use]
pub const fn crc16_with_poly(data: &[u8], poly: u16) -> u16 {
    let mut crc: u16 = 0xFFFF;
    let mut i = 0;
    while i < data.len() {
        #[allow(clippy::indexing_slicing)] // i < data.len()
        let byte = data[i];
        crc ^= byte as u16;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ poly
            } else {
                crc >> 1
            };
            bit += 1;
        }
        i += 1;
    }
    crc
}

const fn make_table(poly: u16) -> [u16; 256] {
    let mut table = [0u16; 256];
    let mut i = 0;
    while i < 256 {
        #[allow(clippy::cast_possible_truncation)] // i < 256
        let mut crc = i as u16;
        let mut bit = 0;
        while bit < 8 {
            crc = if crc & 1 != 0 {
                (crc >> 1) ^ poly
            } else {
                crc >> 1
            };
            bit += 1;
        }
        #[allow(clippy::indexing_slicing)] // i < 256
        {
            table[i] = crc;
        }
        i += 1;
    }
    table
}

/// Lookup table for [`CRC16_POLY_DEFAULT`], computed at compile time.
#[allow(clippy::cast_possible_truncation)] // 0xA001 fits in u16
const CRC16_TABLE: [u16; 256] = make_table(CRC16_POLY_DEFAULT as u16);

/// Table-driven CRC-16 with the protocol's default polynomial.
///
/// ```
/// assert_eq!(iotext_rs::crc16(b"t|3900237526048,d|device_name_005"), 0xC2A7);
/// ```
#[must_use]
pub const fn crc16(data: &[u8]) -> u16 {
    let mut crc: u16 = 0xFFFF;
    let mut i = 0;
    while i < data.len() {
        // Both indices are in bounds: i < data.len(), and a masked byte is < 256.
        #[allow(clippy::indexing_slicing)]
        {
            crc = (crc >> 8) ^ CRC16_TABLE[((crc ^ data[i] as u16) & 0xFF) as usize];
        }
        i += 1;
    }
    crc
}

/// Formats a CRC the way it appears on the wire: four upper-case hex digits.
#[must_use]
pub fn crc16_to_hex(crc: u16) -> String {
    format!("{crc:04X}")
}

/// Pre-1.0 namespace for the CRC helper.
pub struct Tools;

impl Tools {
    /// CRC-16 over the UTF-8 bytes of `data`, as four upper-case hex digits.
    #[deprecated(
        since = "1.3.0",
        note = "use `iotext_rs::crc16(data.as_bytes())` (returns `u16`)"
    )]
    #[must_use]
    pub fn crc16(data: &str, poly: u32) -> String {
        #[allow(clippy::cast_possible_truncation)] // reflected CRC-16 polys fit in u16
        crc16_to_hex(crc16_with_poly(data.as_bytes(), poly as u16))
    }
}
