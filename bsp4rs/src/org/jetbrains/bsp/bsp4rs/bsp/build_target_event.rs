use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetEvent {
    /** The identifier for the changed build target */
    pub target: BuildTargetIdentifier,
    /** The kind of change for this build target */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<BuildTargetEventKind>,
    /** Any additional metadata about what information changed. */
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub data: Option<BuildTargetEventData>,
}
