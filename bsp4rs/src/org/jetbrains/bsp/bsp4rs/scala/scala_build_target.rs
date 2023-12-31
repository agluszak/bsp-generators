use serde::{Deserialize, Serialize};

use crate::*;

/// `ScalaBuildTarget` is a basic data structure that contains scala-specific
/// metadata for compiling a target containing Scala sources.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaBuildTarget {
    /// The Scala organization that is used for a target.
    pub scala_organization: String,
    /// The scala version to compile this target
    pub scala_version: String,
    /// The binary version of scalaVersion.
    /// For example, 2.12 if scalaVersion is 2.12.4.
    pub scala_binary_version: String,
    /// The target platform for this target
    pub platform: ScalaPlatform,
    /// A sequence of Scala jars such as scala-library, scala-compiler and scala-reflect.
    pub jars: Vec<URI>,
    /// The jvm build target describing jdk to be used
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm_build_target: Option<JvmBuildTarget>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_build_target() {
        let test_data = ScalaBuildTarget {
            scala_organization: TEST_STRING.to_string(),
            scala_version: TEST_STRING.to_string(),
            scala_binary_version: TEST_STRING.to_string(),
            platform: ScalaPlatform::default(),
            jars: vec![URI::default()],
            jvm_build_target: Some(JvmBuildTarget::default()),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "scalaOrganization": "test_string",
  "scalaVersion": "test_string",
  "scalaBinaryVersion": "test_string",
  "platform": 1,
  "jars": [
    ""
  ],
  "jvmBuildTarget": {}
}
"#);

        test_deserialization(
            r#"{"scalaOrganization": "test_string", "scalaVersion": "test_string", "scalaBinaryVersion": "test_string", "platform": 1, "jars": [""], "jvmBuildTarget": {}}"#,
            &test_data,
        );
    }
}
