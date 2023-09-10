use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathsItem {
    /** A build target to which output paths item belongs. */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Output paths. */
    #[serde(default)]
    pub output_paths: Vec<OutputPathItem>,
}
