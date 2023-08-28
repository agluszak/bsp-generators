use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    #[serde(default)]
    pub start: Position,
    #[serde(default)]
    pub end: Position,
}
