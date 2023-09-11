use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeBuildTarget {
    #[serde(default)]
    pub changes: Vec<BuildTargetEvent>,
}