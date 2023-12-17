#![allow(deprecated)]

use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod bsp;
pub mod cancel;
pub mod cargo;
pub mod cpp;
pub mod java;
pub mod jvm;
pub mod maven;
pub mod python;
pub mod rust;
pub mod sbt;
pub mod scala;

use bsp::*;
use cancel::*;
use cargo::*;
use cpp::*;
use java::*;
use jvm::*;
use maven::*;
use python::*;
use rust::*;
use sbt::*;
use scala::*;

pub const PROTOCOL_VERSION: &str = "2.2.0";

pub trait Request {
    type Params: DeserializeOwned + Serialize;
    type Result: DeserializeOwned + Serialize;
    const METHOD: &'static str;
}

pub trait Notification {
    type Params: DeserializeOwned + Serialize;
    const METHOD: &'static str;
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherData {
    pub data_kind: String,
    pub data: serde_json::Value,
}

#[cfg(test)]
pub mod tests {
    use super::*;

    use insta::assert_json_snapshot;
    use serde::Deserialize;

    pub const TEST_BOOL: bool = true;
    pub const TEST_INT: i32 = 1;
    pub const TEST_LONG: i64 = 2;
    pub const TEST_STRING: &str = "test_string";

    pub fn test_deserialization<T>(json: &str, expected: &T)
    where
        T: for<'de> Deserialize<'de> + PartialEq + std::fmt::Debug,
    {
        let value = serde_json::from_str::<T>(json).unwrap();
        assert_eq!(&value, expected);
    }

    #[test]
    fn other_data() {
        let test_data = OtherData {
            data_kind: TEST_STRING.to_string(),
            data: serde_json::json!({}),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "dataKind": "test_string",
  "data": {}
}
"#);

        test_deserialization(r#"{"dataKind": "test_string", "data": {}}"#, &test_data);
    }
}
