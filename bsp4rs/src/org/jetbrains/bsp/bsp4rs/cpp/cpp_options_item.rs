use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsItem {
    /// The target identifier for which the options are requested.
    pub target: BuildTargetIdentifier,
    /// Attributes added in the given order to COPTS
    /// before compiling the target.
    /// For example: -Iexternal/gtest/include
    pub copts: Vec<String>,
    /// Attributes prepended with -D
    /// and added to the compile command line
    /// For example: BOOST_FALLTHROUGH
    pub defines: Vec<String>,
    /// Attributes added to the linker command
    /// For example: -pthread
    pub linkopts: Vec<String>,
    /// Create a shared library.
    /// The presence of this flag means that linking occurs with the -shared flag
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linkshared: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn cpp_options_item() {
        let test_data = CppOptionsItem {
            target: BuildTargetIdentifier::default(),
            copts: vec![TEST_STRING.to_string()],
            defines: vec![TEST_STRING.to_string()],
            linkopts: vec![TEST_STRING.to_string()],
            linkshared: Some(TEST_BOOL),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "target": {
    "uri": ""
  },
  "copts": [
    "test_string"
  ],
  "defines": [
    "test_string"
  ],
  "linkopts": [
    "test_string"
  ],
  "linkshared": true
}
   "#
        );

        test_deserialization(
            r#"{"target": {"uri": ""}, "copts": ["test_string"], "defines": ["test_string"], "linkopts": ["test_string"], "linkshared": true}"#,
            &test_data,
        );
    }
}
