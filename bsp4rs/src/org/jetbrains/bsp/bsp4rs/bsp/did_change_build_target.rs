use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeBuildTarget {
    pub changes: Vec<BuildTargetEvent>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn did_change_build_target() {
        let test_data = DidChangeBuildTarget {
            changes: vec![BuildTargetEvent::default()],
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "changes": [
    {
      "target": {
        "uri": ""
      }
    }
  ]
}
   "#
        );

        test_deserialization(r#"{"changes": [{"target": {"uri": ""}}]}"#, &test_data);
    }
}
