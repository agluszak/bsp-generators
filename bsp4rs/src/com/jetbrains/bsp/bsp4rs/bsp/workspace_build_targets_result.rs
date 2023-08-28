use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceBuildTargetsResult {
    /** The build targets in this workspace that
    contain sources with the given language ids. */
    #[serde(default)]
    pub targets: Vec<BuildTarget>,
}
