use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesParams {
    pub targets: Vec<BuildTargetIdentifier>,
    /// An optional number uniquely identifying a client request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn scala_test_classes_params() {
        assert_json_snapshot!(
           ScalaTestClassesParams {targets: vec![BuildTargetIdentifier::default()], origin_id: Some(TEST_STRING.to_string())},
           @r#"
{
  "targets": [
    {
      "uri": ""
    }
  ],
  "originId": "test_string"
}
   "#
        );
    }
}
