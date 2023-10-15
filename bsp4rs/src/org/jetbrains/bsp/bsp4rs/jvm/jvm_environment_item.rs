use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmEnvironmentItem {
    pub target: BuildTargetIdentifier,
    pub classpath: Vec<String>,
    pub jvm_options: Vec<String>,
    pub working_directory: String,
    pub environment_variables: EnvironmentVariables,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub main_classes: Option<Vec<JvmMainClass>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn jvm_environment_item() {
        assert_json_snapshot!(
           JvmEnvironmentItem {target: BuildTargetIdentifier::default(), classpath: vec![TEST_STRING.to_string()], jvm_options: vec![TEST_STRING.to_string()], working_directory: TEST_STRING.to_string(), environment_variables: EnvironmentVariables::default(), main_classes: Some(vec![JvmMainClass::default()])},
           @r#"
{
  "target": {
    "uri": ""
  },
  "classpath": [
    "test_string"
  ],
  "jvmOptions": [
    "test_string"
  ],
  "workingDirectory": "test_string",
  "environmentVariables": {},
  "mainClasses": [
    {
      "className": "",
      "arguments": []
    }
  ]
}
   "#
        );
    }
}
