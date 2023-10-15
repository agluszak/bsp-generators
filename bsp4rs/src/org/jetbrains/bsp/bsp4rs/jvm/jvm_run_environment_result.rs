use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmRunEnvironmentResult {
    pub items: Vec<JvmEnvironmentItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn jvm_run_environment_result() {
        assert_json_snapshot!(
           JvmRunEnvironmentResult {items: vec![JvmEnvironmentItem::default()]},
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
   "#
        );
    }
}
