use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
    /** The text document's URI. */
    pub uri: String,
}
