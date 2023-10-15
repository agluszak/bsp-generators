use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeBuildTarget {
    pub changes: Vec<BuildTargetEvent>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn did_change_build_target() {
        assert_json_snapshot!(
           DidChangeBuildTarget {changes: vec![BuildTargetEvent::default()]},
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
    }
}
