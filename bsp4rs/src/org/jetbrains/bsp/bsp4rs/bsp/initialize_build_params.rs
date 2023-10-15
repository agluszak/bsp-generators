use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildParams {
    /// Name of the client
    pub display_name: String,
    /// The version of the client
    pub version: String,
    /// The BSP version that the client speaks
    pub bsp_version: String,
    /// The rootUri of the workspace
    pub root_uri: URI,
    /// The capabilities of the client
    pub capabilities: BuildClientCapabilities,
    /// Additional metadata about the client
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<InitializeBuildParamsData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn initialize_build_params() {
        assert_json_snapshot!(
           InitializeBuildParams {display_name: TEST_STRING.to_string(), version: TEST_STRING.to_string(), bsp_version: TEST_STRING.to_string(), root_uri: URI::default(), capabilities: BuildClientCapabilities::default(), data: Some(InitializeBuildParamsData::Other(OtherData::default()))},
           @r#"
{
  "displayName": "test_string",
  "version": "test_string",
  "bspVersion": "test_string",
  "rootUri": "",
  "capabilities": {
    "languageIds": []
  },
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
