use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmRunEnvironmentParams {
    pub targets: Vec<BuildTargetIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn jvm_run_environment_params() {
        assert_json_snapshot!(
           JvmRunEnvironmentParams {targets: vec![BuildTargetIdentifier::default()], origin_id: Some(Identifier::default())},
           @r#"
{
  "targets": [
    {
      "uri": ""
    }
  ],
  "originId": ""
}
   "#
        );
    }
}
