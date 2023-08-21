use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathsItem {
    /** A build target to which output paths item belongs. */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Output paths. */
    #[serde(default)]
    pub output_paths: Vec<OutputPathItem>,
}
