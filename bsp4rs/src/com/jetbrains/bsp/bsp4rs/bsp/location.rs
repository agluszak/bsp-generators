use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    #[serde(default)]
    pub uri: URI,
    #[serde(default)]
    pub range: Range,
}
