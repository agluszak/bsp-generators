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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn test_result_data() {
        assert_json_snapshot!(
           TestResultData::Other(OtherData::default()),
           @r#"
{
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
