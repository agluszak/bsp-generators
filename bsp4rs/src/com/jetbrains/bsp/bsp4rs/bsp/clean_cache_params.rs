use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CleanCacheParams {
    /** The build targets to clean. */
    pub targets: Vec<BuildTargetIdentifier>,
}
