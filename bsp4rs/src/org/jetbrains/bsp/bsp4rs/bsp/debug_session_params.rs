use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionParams {
    /// A sequence of build targets affected by the debugging action.
    pub targets: Vec<BuildTargetIdentifier>,
    /// Language-specific metadata for this execution.
    /// See ScalaMainClass as an example.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<DebugSessionParamsData>,
}
