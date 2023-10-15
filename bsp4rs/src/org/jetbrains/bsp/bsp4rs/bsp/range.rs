use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    pub start: Position,
    pub end: Position,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn range() {
        assert_json_snapshot!(
           Range {start: Position::default(), end: Position::default()},
           @r#"
{
  "start": {
    "line": 0,
    "character": 0
  },
  "end": {
    "line": 0,
    "character": 0
  }
}
   "#
        );
    }
}
