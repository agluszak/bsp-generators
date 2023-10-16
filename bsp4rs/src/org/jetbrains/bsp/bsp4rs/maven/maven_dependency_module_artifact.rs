use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenDependencyModuleArtifact {
    /// Path to jar
    pub uri: URI,
    /// Empty or `sources`|`docs`
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn maven_dependency_module_artifact() {
        let test_data = MavenDependencyModuleArtifact {
            uri: URI::default(),
            classifier: Some(TEST_STRING.to_string()),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "uri": "",
  "classifier": "test_string"
}
"#);

        test_deserialization(r#"{"uri": "", "classifier": "test_string"}"#, &test_data);
    }
}
