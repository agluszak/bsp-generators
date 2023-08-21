use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Position {
    /** Line position within a file. First line of a file is 0. */
    #[serde(default)]
    pub line: i32,
    /** Character position within a line. First character of a line is 0. */
    #[serde(default)]
    pub character: i32,
}
