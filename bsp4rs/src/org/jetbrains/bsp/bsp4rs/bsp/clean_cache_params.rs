use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanCacheParams {
    /** The build targets to clean. */
    #[serde(default)]
    pub targets: Vec<BuildTargetIdentifier>,
}
