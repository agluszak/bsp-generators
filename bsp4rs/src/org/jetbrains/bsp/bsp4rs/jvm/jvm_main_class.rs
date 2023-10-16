use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmMainClass {
    pub class_name: String,
    pub arguments: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn jvm_main_class() {
        let test_data = JvmMainClass {
            class_name: TEST_STRING.to_string(),
            arguments: vec![TEST_STRING.to_string()],
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "className": "test_string",
  "arguments": [
    "test_string"
  ]
}
   "#
        );

        test_deserialization(
            r#"{"className": "test_string", "arguments": ["test_string"]}"#,
            &test_data,
        );
    }
}
