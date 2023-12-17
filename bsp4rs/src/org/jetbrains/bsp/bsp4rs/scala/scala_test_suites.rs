use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestSuites {
    /// The fully qualified names of the test classes in this target and the tests in this test classes
    pub suites: Vec<ScalaTestSuiteSelection>,
    /// Additional jvmOptions which will be passed to the forked JVM
    pub jvm_options: Vec<String>,
    /// Enviroment variables should be an array of strings in format KEY=VALUE
    #[deprecated(note = "Use `buildTarget/test` params instead")]
    pub environment_variables: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_test_suites() {
        let test_data = ScalaTestSuites {
            suites: vec![ScalaTestSuiteSelection::default()],
            jvm_options: vec![String::default()],
            environment_variables: vec![String::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "suites": [
    {
      "className": "",
      "tests": []
    }
  ],
  "jvmOptions": [
    ""
  ],
  "environmentVariables": [
    ""
  ]
}
"#);

        test_deserialization(
            r#"{"suites": [{"className": "", "tests": []}], "jvmOptions": [""], "environmentVariables": [""]}"#,
            &test_data,
        );
    }
}
