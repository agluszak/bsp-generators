use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildResult {
    /// Name of the server
    pub display_name: String,
    /// The version of the server
    pub version: String,
    /// The BSP version that the server speaks
    pub bsp_version: String,
    /// The capabilities of the build server
    pub capabilities: BuildServerCapabilities,
    /// Additional metadata about the server
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<InitializeBuildResultData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn initialize_build_result() {
        let test_data = InitializeBuildResult {
            display_name: TEST_STRING.to_string(),
            version: TEST_STRING.to_string(),
            bsp_version: TEST_STRING.to_string(),
            capabilities: BuildServerCapabilities::default(),
            data: Some(InitializeBuildResultData::Other(OtherData::default())),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "displayName": "test_string",
  "version": "test_string",
  "bspVersion": "test_string",
  "capabilities": {},
  "dataKind": "",
  "data": null
}
"#);

        test_deserialization(
            r#"{"displayName": "test_string", "version": "test_string", "bspVersion": "test_string", "capabilities": {}, "dataKind": "", "data": null}"#,
            &test_data,
        );
    }
}
