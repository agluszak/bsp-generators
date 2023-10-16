use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmTestEnvironmentParams {
    pub targets: Vec<BuildTargetIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn jvm_test_environment_params() {
        let test_data = JvmTestEnvironmentParams {
            targets: vec![BuildTargetIdentifier::default()],
            origin_id: Some(Identifier::default()),
        };

        assert_json_snapshot!(
           test_data,
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

        test_deserialization(r#"{"targets": [{"uri": ""}], "originId": ""}"#, &test_data);
    }
}
