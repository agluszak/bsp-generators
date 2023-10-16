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
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn location() {
        let test_data = Location {
            uri: URI::default(),
            range: Range::default(),
        };

        assert_json_snapshot!(
           test_data,
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

        test_deserialization(
            r#"{"uri": "", "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}}"#,
            &test_data,
        );
    }
}
