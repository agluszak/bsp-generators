use serde::{Deserialize, Serialize};

use crate::*;

/// `MavenDependencyModule` is a basic data structure that contains maven-like
/// metadata. This metadata is embedded in the `data: Option[Json]` field of the `DependencyModule` definition, when the `dataKind` field contains "maven".
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenDependencyModule {
    pub organization: String,
    pub name: String,
    pub version: String,
    /// List of module's artifacts with different classifiers.
    /// For example: [
    ///   {uri = "../scala-library-2.13.5.jar"},
    ///   {uri = "../scala-library-2.13.5-sources.jar", classifier = "sources"}
    /// ]
    pub artifacts: Vec<MavenDependencyModuleArtifact>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn maven_dependency_module() {
        let test_data = MavenDependencyModule {
            organization: TEST_STRING.to_string(),
            name: TEST_STRING.to_string(),
            version: TEST_STRING.to_string(),
            artifacts: vec![MavenDependencyModuleArtifact::default()],
            scope: Some(TEST_STRING.to_string()),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "organization": "test_string",
  "name": "test_string",
  "version": "test_string",
  "artifacts": [
    {
      "uri": ""
    }
  ],
  "scope": "test_string"
}
   "#
        );

        test_deserialization(
            r#"{"organization": "test_string", "name": "test_string", "version": "test_string", "artifacts": [{"uri": ""}], "scope": "test_string"}"#,
            &test_data,
        );
    }
}
