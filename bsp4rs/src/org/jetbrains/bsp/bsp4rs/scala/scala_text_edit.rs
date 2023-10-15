use serde::{Deserialize, Serialize};

use crate::*;

/// A textual edit applicable to a text document.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTextEdit {
    /// The range of the text document to be manipulated. To insert
    /// text into a document create a range where start === end.
    pub range: Range,
    /// The string to be inserted. For delete operations use an
    /// empty string.
    pub new_text: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn scala_text_edit() {
        assert_json_snapshot!(
           ScalaTextEdit {range: Range::default(), new_text: TEST_STRING.to_string()},
           @r#"
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
  "newText": "test_string"
}
   "#
        );
    }
}
