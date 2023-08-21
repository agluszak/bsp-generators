use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileProvider {
    #[serde(default)]
    pub language_ids: Vec<LanguageId>,
}
