use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuildClientCapabilities {
    /** The languages that this client supports.
    The ID strings for each language is defined in the LSP.
    The server must never respond with build targets for other
    languages than those that appear in this list. */
    pub language_ids: Vec<String>,
}
