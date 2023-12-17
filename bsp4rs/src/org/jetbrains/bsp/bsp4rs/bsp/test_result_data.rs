use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTestResultData {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestResultData {
    Named(NamedTestResultData),
    Other(OtherData),
}

impl TestResultData {}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;

    use serde_json::json;

    #[test]
    fn test_result_data() {
        assert_compact_json_snapshot!(
           TestResultData::Other(OtherData { data: json!({}), ..OtherData::default()}),
           @r#"{"dataKind": "", "data": {}}"#
        );
    }
}
