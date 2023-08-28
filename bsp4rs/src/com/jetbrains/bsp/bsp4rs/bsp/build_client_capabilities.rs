use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildClientCapabilities {
    /** The languages that this client supports.
    The ID strings for each language is defined in the LSP.
    The server must never respond with build targets for other
    languages than those that appear in this list. */
    #[serde(default)]
    pub language_ids: Vec<LanguageId>,
}
