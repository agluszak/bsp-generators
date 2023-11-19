use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    /// The range's start position.
    pub start: Position,
    /// The range's end position.
    pub end: Position,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn range() {
        let test_data = Range {
            start: Position::default(),
            end: Position::default(),
        };

        assert_json_snapshot!(test_data,
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
"#);

        test_deserialization(
            r#"{"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}"#,
            &test_data,
        );
    }
}
