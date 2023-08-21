use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeBuildTarget {
    #[serde(default)]
    pub changes: Vec<BuildTargetEvent>,
}
