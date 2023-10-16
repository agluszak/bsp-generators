use serde::{Deserialize, Serialize};

use crate::*;

/// `ScalaTestParams` contains scala-specific metadata for testing Scala targets.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestParams {
    /// The test classes to be run in this test execution.
    /// It is the result of `buildTarget/scalaTestClasses`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_classes: Option<Vec<ScalaTestClassesItem>>,
    /// The JVM options to run tests with. They replace any options
    /// that are defined by the build server if defined.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub jvm_options: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_test_params() {
        let test_data = ScalaTestParams {
            test_classes: Some(vec![ScalaTestClassesItem::default()]),
            jvm_options: Some(vec![TEST_STRING.to_string()]),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "testClasses": [
    {
      "target": {
        "uri": ""
      },
      "classes": []
    }
  ],
  "jvmOptions": [
    "test_string"
  ]
}
   "#
        );

        test_deserialization(
            r#"{"testClasses": [{"target": {"uri": ""}, "classes": []}], "jvmOptions": ["test_string"]}"#,
            &test_data,
        );
    }
}
