use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanCacheParams {
    /** The build targets to clean. */
    pub targets: Vec<BuildTargetIdentifier>,
}
