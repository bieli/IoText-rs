use std::*;

use iotext_rs::*;
#[warn(unused_imports)]
use rust_decimal::Decimal;
#[warn(unused_imports)]
use std::str::FromStr;

fn main() {
    let iotext_data_row = parse_iotext_str(MSG_EXAMPLE);
    println!("iotext_data_row.timestamp_mut: {:#?}", iotext_data_row)
}

/*
STDOUT:

part: t|3900237526042
item_part: ["t", "3900237526042"]
                        TIMESTAMP_MILIS: TimeUnixMilis(3900237526042)
                context: 3900237526042
part: d|device_name_001
item_part: ["d", "device_name_001"]
                        DEVICE_ID: DeviceId("device_name_001")
                context: device_name_001
part: m|val_water_level1=i:42
item_part: ["m", "val_water_level1=i:42"]
        metric: val_water_level1=i:42
        metric_parts: ["val_water_level1", "i:42"]
                metric_parts_values: ["i", "42"]
                        IntegerItemType: IntegerItemType(42)
part: m|light_on=b:1
item_part: ["m", "light_on=b:1"]
        metric: light_on=b:1
        metric_parts: ["light_on", "b:1"]
                metric_parts_values: ["b", "1"]
                        BoolItemType: BoolItemType(true)
part: m|bulb_on=b:0
item_part: ["m", "bulb_on=b:0"]
        metric: bulb_on=b:0
        metric_parts: ["bulb_on", "b:0"]
                metric_parts_values: ["b", "0"]
                        BoolItemType: BoolItemType(false)
part: m|msg_machine_01=t:hello
item_part: ["m", "msg_machine_01=t:hello"]
        metric: msg_machine_01=t:hello
        metric_parts: ["msg_machine_01", "t:hello"]
                metric_parts_values: ["t", "hello"]
                        BoolItemType: TextItemType("hello")
part: m|wind_speed=d:1234.5678
item_part: ["m", "wind_speed=d:1234.5678"]
        metric: wind_speed=d:1234.5678
        metric_parts: ["wind_speed", "d:1234.5678"]
                metric_parts_values: ["d", "1234.5678"]
                        DecimalItemType: DecimalItemType(1234.5678)
iotext_data_row.timestamp_mut: IotextDataRow {
    timestamp: Item {
        value: TimeUnixMilis(
            3900237526042,
        ),
    },
    device_id: Item {
        value: DeviceId(
            "device_name_001",
        ),
    },
    metrics: Some(
        [
            MetricDataItem {
                name: "val_water_level1",
                value: IntegerItemType(
                    42,
                ),
            },
            MetricDataItem {
                name: "light_on",
                value: BoolItemType(
                    true,
                ),
            },
            MetricDataItem {
                name: "bulb_on",
                value: BoolItemType(
                    false,
                ),
            },
            MetricDataItem {
                name: "msg_machine_01",
                value: TextItemType(
                    "hello",
                ),
            },
            MetricDataItem {
                name: "wind_speed",
                value: DecimalItemType(
                    1234.5678,
                ),
            },
        ],
    ),
}



*/

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_extract_metric_value_type_integer() {
        const METRIC_DATA_TYPE: &str = "i";
        const METRIC_DATA_VALUE: &str = "42";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::IntegerItemType(42));
    }

    #[test]
    fn test_extract_metric_value_type_decimal() {
        const METRIC_DATA_TYPE: &str = "d";
        const METRIC_DATA_VALUE: &str = "42.123";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(
            result,
            MetricValueType::DecimalItemType(Decimal::from_str(METRIC_DATA_VALUE).unwrap())
        );
    }

    #[test]
    fn test_extract_metric_value_type_bool_true() {
        const METRIC_DATA_TYPE: &str = "b";
        const METRIC_DATA_VALUE: &str = "1";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::BoolItemType(true));
    }

    #[test]
    fn test_extract_metric_value_type_bool_false() {
        const METRIC_DATA_TYPE: &str = "b";
        const METRIC_DATA_VALUE: &str = "0";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(result, MetricValueType::BoolItemType(false));
    }

    #[test]
    fn test_extract_metric_value_type_text() {
        const METRIC_DATA_TYPE: &str = "t";
        const METRIC_DATA_VALUE: &str = "hello";

        let result = extract_metric_value_type(METRIC_DATA_TYPE, METRIC_DATA_VALUE);

        assert_eq!(
            result,
            MetricValueType::TextItemType(METRIC_DATA_VALUE.to_string())
        );
    }
}
