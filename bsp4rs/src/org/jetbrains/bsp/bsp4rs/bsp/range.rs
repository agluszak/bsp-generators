use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Range {
    #[serde(default)]
    pub start: Position,
    #[serde(default)]
    pub end: Position,
}
