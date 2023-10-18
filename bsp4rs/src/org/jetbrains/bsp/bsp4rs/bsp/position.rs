use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Line position in a document (zero-based).
    pub line: i32,
    /// Character offset on a line in a document (zero-based)
    ///
    /// If the character value is greater than the line length it defaults back
    /// to the line length.
    pub character: i32,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn position() {
        let test_data = Position {
            line: TEST_INT,
            character: TEST_INT,
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "line": 1,
  "character": 1
}
"#);

        test_deserialization(r#"{"line": 1, "character": 1}"#, &test_data);
    }
}
