use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathsItem {
    /** A build target to which output paths item belongs. */
    pub target: BuildTargetIdentifier,
    /** Output paths. */
    pub output_paths: Vec<OutputPathItem>,
}
