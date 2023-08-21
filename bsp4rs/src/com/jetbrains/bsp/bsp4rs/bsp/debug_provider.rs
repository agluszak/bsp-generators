use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugProvider {
    #[serde(default)]
    pub language_ids: Vec<LanguageId>,
}
