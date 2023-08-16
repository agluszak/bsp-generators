use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceBuildTargetsResult {
    /** The build targets in this workspace that
    contain sources with the given language ids. */
    pub targets: Vec<BuildTarget>,
}
