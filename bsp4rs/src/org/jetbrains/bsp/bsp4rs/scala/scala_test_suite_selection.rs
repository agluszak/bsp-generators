use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestSuiteSelection {
    /// Fully qualified name of the test suite class
    pub class_name: String,
    /// List of tests which should be run within this test suite.
    /// Empty collection means that all of them are supposed to be executed.
    pub tests: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_test_suite_selection() {
        let test_data = ScalaTestSuiteSelection {
            class_name: TEST_STRING.to_string(),
            tests: vec![TEST_STRING.to_string()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "className": "test_string",
  "tests": [
    "test_string"
  ]
}
"#);

        test_deserialization(
            r#"{"className": "test_string", "tests": ["test_string"]}"#,
            &test_data,
        );
    }
}
