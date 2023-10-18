use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTestFinishData {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestFinishData {
    Named(NamedTestFinishData),
    Other(OtherData),
}

impl TestFinishData {}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;

    #[test]
    fn test_finish_data() {
        assert_compact_json_snapshot!(
           TestFinishData::Other(OtherData::default()),
           @r#"{"dataKind": "", "data": null}"#
        );
    }
}
