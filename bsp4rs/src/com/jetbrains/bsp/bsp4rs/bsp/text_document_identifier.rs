use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
    /** The text document's URI. */
    #[serde(default)]
    pub uri: URI,
}
