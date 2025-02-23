pub struct Tools;

use std::*;

pub const CRC16_POLY_DEFAULT: u32 = 0xA001;

impl Tools {
    pub fn crc16(data: &str, poly: u32) -> String {
        let mut crc: u32 = 0xFFFF;
        for byte in data.bytes() {
            crc ^= byte as u32;
            for _ in 0..8 {
                crc = if (crc & 0x0001) != 0 {
                    (crc >> 1) ^ poly
                } else {
                    crc >> 1
                };
            }
        }
        let hv = format!("{:X}", crc);
        let blueprint = "0000".to_string();
        if hv.is_empty() {
            blueprint
        } else {
            let len_diff = blueprint.len().saturating_sub(hv.len());
            format!("{}{}", &blueprint[..len_diff], hv)
        }
    }
}
