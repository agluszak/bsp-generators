use serde::{Deserialize, Serialize};

use crate::*;

/** A textual edit applicable to a text document. */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTextEdit {
    /** The range of the text document to be manipulated. To insert
    text into a document create a range where start === end. */
    pub range: Range,
    /** The string to be inserted. For delete operations use an
    empty string. */
    pub new_text: String,
}