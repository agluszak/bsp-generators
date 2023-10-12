use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkspaceBuildTargetsResult {
    /// The build targets in this workspace that
    /// contain sources with the given language ids.
    pub targets: Vec<BuildTarget>,
}

#[cfg(test)]
mod tests {}
