use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavacOptionsItem {
    pub target: BuildTargetIdentifier,
    /// Additional arguments to the compiler.
    /// For example, -deprecation.
    pub options: Vec<String>,
    /// The dependency classpath for this target, must be
    /// identical to what is passed as arguments to
    /// the -classpath flag in the command line interface
    /// of javac.
    pub classpath: Vec<String>,
    /// The output directory for classfiles produced by this target
    pub class_directory: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn javac_options_item() {
        let test_data = JavacOptionsItem {
            target: BuildTargetIdentifier::default(),
            options: vec![TEST_STRING.to_string()],
            classpath: vec![TEST_STRING.to_string()],
            class_directory: TEST_STRING.to_string(),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "target": {
    "uri": ""
  },
  "options": [
    "test_string"
  ],
  "classpath": [
    "test_string"
  ],
  "classDirectory": "test_string"
}
   "#
        );

        test_deserialization(
            r#"{"target": {"uri": ""}, "options": ["test_string"], "classpath": ["test_string"], "classDirectory": "test_string"}"#,
            &test_data,
        );
    }
}
