use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedInitializeBuildResultData {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializeBuildResultData {
    Named(NamedInitializeBuildResultData),
    Other(OtherData),
}

impl InitializeBuildResultData {}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;

    #[test]
    fn initialize_build_result_data() {
        assert_compact_json_snapshot!(
           InitializeBuildResultData::Other(OtherData::default()),
           @r#"{"dataKind": "", "data": null}"#
        );
    }
}
