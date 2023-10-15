use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub uri: URI,
    pub range: Range,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn location() {
        assert_json_snapshot!(
           Location {uri: URI::default(), range: Range::default()},
           @r#"
{
  "uri": "",
  "range": {
    "start": {
      "line": 0,
      "character": 0
    },
    "end": {
      "line": 0,
      "character": 0
    }
  }
}
   "#
        );
    }
}
