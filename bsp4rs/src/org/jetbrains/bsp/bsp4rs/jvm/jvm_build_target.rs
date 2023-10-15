use serde::{Deserialize, Serialize};

use crate::*;

/// `JvmBuildTarget` is a basic data structure that contains jvm-specific
/// metadata, specifically JDK reference.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmBuildTarget {
    /// Uri representing absolute path to jdk
    /// For example: file:///usr/lib/jvm/java-8-openjdk-amd64
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_home: Option<URI>,
    /// The java version this target is supposed to use.
    /// For example: 1.8
    #[serde(skip_serializing_if = "Option::is_none")]
    pub java_version: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn jvm_build_target() {
        assert_json_snapshot!(
           JvmBuildTarget {java_home: Some(URI::default()), java_version: Some(TEST_STRING.to_string())},
           @r#"
{
  "javaHome": "",
  "javaVersion": "test_string"
}
   "#
        );
    }
}
