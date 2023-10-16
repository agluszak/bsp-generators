use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /// Line position within a file. First line of a file is 0.
    pub line: i32,
    /// Character position within a line. First character of a line is 0.
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

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "line": 1,
  "character": 1
}
   "#
        );

        test_deserialization(r#"{"line": 1, "character": 1}"#, &test_data);
    }
}
