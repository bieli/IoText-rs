use iotext_rs::Tools;
use iotext_rs::CRC16_POLY_DEFAULT;

#[cfg(test)]
mod tools_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_should_crc16_basic_message() {
        let expected_result = "C2A7";
        let iot_ext_proto_test_msg: String = "t|3900237526048,d|device_name_005".to_string();

        let result = Tools::crc16(&iot_ext_proto_test_msg, CRC16_POLY_DEFAULT);

        assert_eq!(result, expected_result);
    }

    #[test]
    fn test_should_crc16_message_with_items() {
        let expected_result = "4C5A";
        let iot_ext_proto_test_msg: String =
            "t|123123123123,d|device_one,m|value=d:123.321".to_string();

        let result = Tools::crc16(&iot_ext_proto_test_msg, CRC16_POLY_DEFAULT);

        assert_eq!(result, expected_result);
    }
}
