use crate::proto::opentelemetry::proto::common::v1::any_value::Value;
use crate::proto::opentelemetry::proto::common::v1::{
    AnyValue, ArrayValue, KeyValue, KeyValueList,
};
use opentelemetry::api::LabelSet;
use opentelemetry::sdk::EvictedHashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

impl From<EvictedHashMap> for KeyValueList {
    fn from(ehm: EvictedHashMap) -> Self {
        KeyValueList {
            values: ehm
                .into_iter()
                .map(|(key, value)| KeyValue {
                    key: key.as_str().to_string(),
                    value: Some(AnyValue::from(value)),
                })
                .collect(),
        }
    }
}

impl From<Vec<opentelemetry::api::KeyValue>> for KeyValueList {
    fn from(kvs: Vec<opentelemetry::api::KeyValue>) -> Self {
        KeyValueList {
            values: kvs
                .into_iter()
                .map(|api_kv| KeyValue {
                    key: api_kv.key.as_str().to_string(),
                    value: Some(AnyValue::from(api_kv.value)),
                })
                .collect(),
        }
    }
}

impl From<opentelemetry::api::Value> for AnyValue {
    fn from(value: opentelemetry::api::Value) -> Self {
        AnyValue {
            value: match value {
                opentelemetry::api::Value::Bool(val) => Some(Value::BoolValue(val)),
                opentelemetry::api::Value::I64(val) => Some(Value::IntValue(val)),
                opentelemetry::api::Value::U64(val) => Some(Value::IntValue(val as i64)),
                opentelemetry::api::Value::F64(val) => Some(Value::DoubleValue(val)),
                opentelemetry::api::Value::String(val) => Some(Value::StringValue(val)),
                opentelemetry::api::Value::Bytes(_val) => None,
                opentelemetry::api::Value::Array(vals) => Some(Value::ArrayValue(ArrayValue {
                    values: vals.into_iter().map(Into::into).collect(),
                })),
            },
        }
    }
}

impl LabelSet for KeyValue {}

pub(crate) fn to_nanos(time: SystemTime) -> u64 {
    time.duration_since(UNIX_EPOCH)
        .unwrap_or_else(|_| Duration::from_secs(0))
        .as_nanos() as u64
}
