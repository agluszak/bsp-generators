use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModule {
    /// Module name
    pub name: String,
    /// Module version
    pub version: String,
    /// Language-specific metadata about this module.
    /// See MavenDependencyModule as an example.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<DependencyModuleData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    use serde_json::json;

    #[test]
    fn dependency_module() {
        let test_data = DependencyModule {
            name: TEST_STRING.to_string(),
            version: TEST_STRING.to_string(),
            data: Some(DependencyModuleData::Other(OtherData {
                data: json!({}),
                ..OtherData::default()
            })),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "name": "test_string",
  "version": "test_string",
  "dataKind": "",
  "data": {}
}
"#);

        test_deserialization(
            r#"{"name": "test_string", "version": "test_string", "dataKind": "", "data": {}}"#,
            &test_data,
        );
    }
}
