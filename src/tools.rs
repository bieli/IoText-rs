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

/// Slice-by-8 tables: `TABLES[k][b]` is the CRC contribution of byte `b` followed by `k`
/// zero bytes. Computed at compile time (8 x 256 x 2 bytes = 4 KiB).
const fn make_slice_tables(poly: u16) -> [[u16; 256]; 8] {
    let base = make_table(poly);
    let mut tables = [[0u16; 256]; 8];
    let mut i = 0;
    // Index arithmetic below is bounded: i < 256, k < 8, `& 0xFF` < 256.
    #[allow(clippy::indexing_slicing)]
    while i < 256 {
        tables[0][i] = base[i];
        let mut k = 1;
        while k < 8 {
            let prev = tables[k - 1][i];
            tables[k][i] = (prev >> 8) ^ base[(prev & 0xFF) as usize];
            k += 1;
        }
        i += 1;
    }
    tables
}

#[allow(clippy::cast_possible_truncation)] // 0xA001 fits in u16
const CRC16_TABLES: [[u16; 256]; 8] = make_slice_tables(CRC16_POLY_DEFAULT as u16);

/// CRC-16 with the protocol's default polynomial (slice-by-8, tables built at compile time).
///
/// ```
/// assert_eq!(iotext_rs::crc16(b"t|3900237526048,d|device_name_005"), 0xC2A7);
/// ```
#[must_use]
pub const fn crc16(data: &[u8]) -> u16 {
    let t = &CRC16_TABLES;
    let mut crc: u16 = 0xFFFF;
    let mut i = 0;
    // All indices are bounded: `i + 7 < data.len()` / `i < data.len()`, table indices < 256.
    #[allow(clippy::indexing_slicing)]
    {
        // Eight independent lookups per step break the byte-to-byte dependency chain.
        while i + 8 <= data.len() {
            let x = crc ^ (data[i] as u16 | ((data[i + 1] as u16) << 8));
            crc = t[7][(x & 0xFF) as usize]
                ^ t[6][(x >> 8) as usize]
                ^ t[5][data[i + 2] as usize]
                ^ t[4][data[i + 3] as usize]
                ^ t[3][data[i + 4] as usize]
                ^ t[2][data[i + 5] as usize]
                ^ t[1][data[i + 6] as usize]
                ^ t[0][data[i + 7] as usize];
            i += 8;
        }
        while i < data.len() {
            crc = (crc >> 8) ^ t[0][((crc ^ data[i] as u16) & 0xFF) as usize];
            i += 1;
        }
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
