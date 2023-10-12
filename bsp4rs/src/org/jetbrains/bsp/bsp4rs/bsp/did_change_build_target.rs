use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DidChangeBuildTarget {
    pub changes: Vec<BuildTargetEvent>,
}

#[cfg(test)]
mod tests {}
