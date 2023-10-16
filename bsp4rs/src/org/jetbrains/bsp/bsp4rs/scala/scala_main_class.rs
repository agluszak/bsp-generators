use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClass {
    /// The main class to run.
    #[serde(rename = "class")]
    pub class_name: String,
    /// The user arguments to the main entrypoint.
    #[deprecated(note = "Use `buildTarget/run` params instead")]
    pub arguments: Vec<String>,
    /// The jvm options for the application.
    pub jvm_options: Vec<String>,
    /// The environment variables for the application.
    #[deprecated(note = "Use `buildTarget/run` params instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub environment_variables: Option<Vec<String>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_main_class() {
        let test_data = ScalaMainClass {
            class_name: TEST_STRING.to_string(),
            arguments: vec![TEST_STRING.to_string()],
            jvm_options: vec![TEST_STRING.to_string()],
            environment_variables: Some(vec![TEST_STRING.to_string()]),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "class": "test_string",
  "arguments": [
    "test_string"
  ],
  "jvmOptions": [
    "test_string"
  ],
  "environmentVariables": [
    "test_string"
  ]
}
"#);

        test_deserialization(
            r#"{"class": "test_string", "arguments": ["test_string"], "jvmOptions": ["test_string"], "environmentVariables": ["test_string"]}"#,
            &test_data,
        );
    }
}
