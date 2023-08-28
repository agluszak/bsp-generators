use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathsResult {
    #[serde(default)]
    pub items: Vec<OutputPathsItem>,
}
