use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionParams {
    /** A sequence of build targets affected by the debugging action. */
    #[serde(default)]
    pub targets: Vec<BuildTargetIdentifier>,
    /** Language-specific metadata for this execution.
    See ScalaMainClass as an example. */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<DebugSessionParamsData>,
}
