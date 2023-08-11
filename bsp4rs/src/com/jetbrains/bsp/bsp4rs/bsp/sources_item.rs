use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct SourcesItem {
    pub target: BuildTargetIdentifier,
    /** The text documents or and directories that belong to this build target. */
    pub sources: Vec<SourceItem>,
    /** The root directories from where source files should be relativized.
    Example: ["file://Users/name/dev/metals/src/main/scala"] */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub roots: Vec<String>,
}