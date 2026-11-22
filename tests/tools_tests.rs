//! CRC-16 tests.

use iotext_rs::{crc16, crc16_to_hex};

#[cfg(test)]
mod tools_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_crc16_basic_message() {
        let expected_result = "C2A7";
        let iot_ext_proto_test_msg: String = "t|3900237526048,d|device_name_005".to_string();

        let result = crc16_to_hex(crc16(iot_ext_proto_test_msg.as_bytes()));

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_should_crc16_message_with_items() {
        let expected_result = "4C5A";
        let iot_ext_proto_test_msg: String =
            "t|123123123123,d|device_one,m|value=d:123.321".to_string();

        let result = crc16_to_hex(crc16(iot_ext_proto_test_msg.as_bytes()));

        assert_eq!(result, expected_result);
    }

    #[test]
    #[allow(deprecated)]
    fn test_deprecated_tools_crc16_matches_new_api() {
        let msg = "t|3900237526048,d|device_name_005";
        assert_eq!(
            iotext_rs::Tools::crc16(msg, iotext_rs::CRC16_POLY_DEFAULT),
            crc16_to_hex(crc16(msg.as_bytes()))
        );
    }

    #[test]
    fn test_crc16_is_const_evaluable() {
        const CRC: u16 = crc16(b"t|3900237526048,d|device_name_005");
        assert_eq!(CRC, 0xC2A7);
        assert_eq!(crc16(b""), 0xFFFF);
    }
}
