use rust_decimal::prelude::*;

use iotext_rs::IoTextData;
use iotext_rs::IoTextDataRow;
use iotext_rs::ItemTypeEnum;
use iotext_rs::MetricValueType;

#[cfg(test)]
mod lib_tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_metric_value_type_integer() {
        let data_obj = IoTextDataRow::default();
        const METRIC_DATA_TYPE: &str = "i";
        const METRIC_DATA_VALUE: &str = "42";

        let result = data_obj.extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::IntegerItemType(42));
    }

    #[test]
    fn test_extract_metric_value_type_decimal() {
        let data_obj = IoTextDataRow::default();
        const METRIC_DATA_TYPE: &str = "d";
        const METRIC_DATA_VALUE: &str = "42.123";

        let result = data_obj.extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(
            result,
            MetricValueType::DecimalItemType(Decimal::from_str(METRIC_DATA_VALUE).unwrap())
        );
    }

    #[test]
    fn test_extract_metric_value_type_bool_true() {
        let data_obj = IoTextDataRow::default();
        const METRIC_DATA_TYPE: &str = "b";
        const METRIC_DATA_VALUE: &str = "1";

        let result = data_obj.extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::BoolItemType(true));
    }

    #[test]
    fn test_extract_metric_value_type_bool_false() {
        let data_obj = IoTextDataRow::default();
        const METRIC_DATA_TYPE: &str = "b";
        const METRIC_DATA_VALUE: &str = "0";

        let result = data_obj.extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::BoolItemType(false));
    }

    #[test]
    fn test_extract_metric_value_type_text() {
        let data_obj = IoTextDataRow::default();
        const METRIC_DATA_TYPE: &str = "t";
        const METRIC_DATA_VALUE: &str = "hello";

        let result = data_obj.extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(
            result,
            MetricValueType::TextItemType(METRIC_DATA_VALUE.to_string())
        );
    }

    #[test]
    fn test_dump_iotext_to_str_without_measurements_without_crc16() {
        let data_obj = IoTextDataRow::default();
        let expected: String = "t|3900237526042,d|device_name_001".to_string();
        let mut iotext_data_row = IoTextDataRow::default();

        let s = iotext_data_row.timestamp_mut();
        s.value = ItemTypeEnum::TimeUnixMilis(3900237526042);

        let s = iotext_data_row.device_id_mut();
        s.value = ItemTypeEnum::DeviceId("device_name_001".to_string());

        let result: String = data_obj.dump_iotext_to_str(&iotext_data_row, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dump_iotext_to_str_with_measurements_without_crc16() {
        let data_obj = IoTextDataRow::default();
        let expected: String = "t|3900237526044,d|device_name_002,m|val1i=i:123,m|val2d=d:123.45,m|valb1=b:1,m|valb0=b:0,m|valt=t:abc".to_string();
        let iotext_data_row = data_obj.parse_iotext_str(&expected);

        let result: String = data_obj.dump_iotext_to_str(&iotext_data_row, false);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_iotext_str_for_timestamp_without_measurements() {
        let expected_timestamp: u64 = 3900237526044;
        let data_obj = IoTextDataRow::default();
        let iot_ext_proto_test_msg: String = "t|3900237526044,d|device_name_004".to_string();
        let result = data_obj.parse_iotext_str(&iot_ext_proto_test_msg);

        let result_timestamp: u64 = match result.get_timestamp() {
            ItemTypeEnum::TimeUnixMilis(value) => value.to_owned(),
            _ => todo!(),
        };

        assert_eq!(result_timestamp, expected_timestamp);
    }

    #[test]
    fn test_parse_iotext_str_for_device_id_without_measurements() {
        let expected_device_id: String = "device_name_005".to_string();
        let data_obj = IoTextDataRow::default();
        let iot_ext_proto_test_msg: String = "t|3900237526045,d|device_name_005".to_string();
        let result = data_obj.parse_iotext_str(&iot_ext_proto_test_msg);

        let result_device_id: String = match result.get_device_id() {
            ItemTypeEnum::DeviceId(value) => value.to_owned(),
            _ => todo!(),
        };

        assert_eq!(result_device_id, expected_device_id);
    }

    #[test]
    fn test_parse_iotext_str_for_device_id_without_measurements_with_crc16() {
        let expected_crc16: String = "c|E4D5".to_string();
        let data_obj = IoTextDataRow::default();
        let iot_ext_proto_test_msg: String = "t|3900237526045,d|device_name_005,c|E4D5".to_string();
        let result = data_obj.parse_iotext_str(&iot_ext_proto_test_msg);

        let optional_crc16 = result.get_crc16();
        let result_crc16: String = optional_crc16.as_ref().unwrap().to_string();
        // let val1 = optional_crc16.as_ref().unwrap();
        // let result_crc16: String = match val1 {
        //     value => value.value.to_string(),
        // };

        assert_eq!(result_crc16, expected_crc16);
    }

    #[test]
    fn test_dump_iotext_to_str_without_measurements_with_crc16() {
        let data_obj = IoTextDataRow::default();
        let expected: String = "t|3900237526042,d|DEV_NAME_002,c|6E24".to_string();
        let mut iotext_data_row = IoTextDataRow::default();

        let s = iotext_data_row.timestamp_mut();
        s.value = ItemTypeEnum::TimeUnixMilis(3900237526042);

        let s = iotext_data_row.device_id_mut();
        s.value = ItemTypeEnum::DeviceId("DEV_NAME_002".to_string());

        // let crc16_mut_opt = iotext_data_row.crc16_mut();
        //&crc16_mut_opt.unwrap().value = ItemTypeEnum::Crc("07A3".to_string());
        //crc16_mut_opt.unwrap().value = ItemTypeEnum::Crc("07A3".to_string());

        let result: String = data_obj.dump_iotext_to_str(&iotext_data_row, true);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_dump_iotext_to_str_with_measurements_with_crc16() {
        let data_obj = IoTextDataRow::default();
        let expected: String =
            "t|3900237526042,d|DEV_NAME_002,m|example_metric_name=d:12.07,c|3E3C".to_string();
        let iotext_data_row = data_obj.parse_iotext_str(&expected);

        let result: String = data_obj.dump_iotext_to_str(&iotext_data_row, true);

        assert_eq!(result, expected);
    }
}
