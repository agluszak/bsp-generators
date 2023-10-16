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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn initialize_build_params_data() {
        assert_json_snapshot!(
           InitializeBuildParamsData::Other(OtherData::default()),
           @r#"
{
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
