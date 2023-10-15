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
        assert_json_snapshot!(
           JvmMainClass {class_name: TEST_STRING.to_string(), arguments: vec![TEST_STRING.to_string()]},
           @r#"
{
  "className": "test_string",
  "arguments": [
    "test_string"
  ]
}
   "#
        );
    }
}
