use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmRunEnvironmentResult {
    pub items: Vec<JvmEnvironmentItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn jvm_run_environment_result() {
        let test_data = JvmRunEnvironmentResult {
            items: vec![JvmEnvironmentItem::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "classpath": [],
      "jvmOptions": [],
      "workingDirectory": "",
      "environmentVariables": {}
    }
  ]
}
"#);

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "classpath": [], "jvmOptions": [], "workingDirectory": "", "environmentVariables": {}}]}"#,
            &test_data,
        );
    }
}
