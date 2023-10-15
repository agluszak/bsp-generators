use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesItem {
    /// The build target that contains the test classes.
    pub target: BuildTargetIdentifier,
    /// Name of the the framework to which classes belong.
    /// It's optional in order to maintain compatibility, however it is expected
    /// from the newer implementations to not leave that field unspecified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /// The fully qualified names of the test classes in this target
    pub classes: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn scala_test_classes_item() {
        assert_json_snapshot!(
           ScalaTestClassesItem {target: BuildTargetIdentifier::default(), framework: Some(TEST_STRING.to_string()), classes: vec![TEST_STRING.to_string()]},
           @r#"
{
  "target": {
    "uri": ""
  },
  "framework": "test_string",
  "classes": [
    "test_string"
  ]
}
   "#
        );
    }
}
