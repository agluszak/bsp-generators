use serde::{Deserialize, Serialize};

use crate::*;

/// A workspace edit represents changes to many resources managed in the workspace.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaWorkspaceEdit {
    pub changes: Vec<ScalaTextEdit>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn scala_workspace_edit() {
        assert_json_snapshot!(
           ScalaWorkspaceEdit {changes: vec![ScalaTextEdit::default()]},
           @r#"
{
  "changes": [
    {
      "range": {
        "start": {
          "line": 0,
          "character": 0
        },
        "end": {
          "line": 0,
          "character": 0
        }
      },
      "newText": ""
    }
  ]
}
   "#
        );
    }
}
