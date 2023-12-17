use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedInitializeBuildParamsData {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializeBuildParamsData {
    Named(NamedInitializeBuildParamsData),
    Other(OtherData),
}

impl InitializeBuildParamsData {}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;

    use serde_json::json;

    #[test]
    fn initialize_build_params_data() {
        assert_compact_json_snapshot!(
           InitializeBuildParamsData::Other(OtherData { data: json!({}), ..OtherData::default()}),
           @r#"{"dataKind": "", "data": {}}"#
        );
    }
}
