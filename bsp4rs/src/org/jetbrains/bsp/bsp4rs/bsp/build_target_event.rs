use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetEvent {
    /** The identifier for the changed build target */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** The kind of change for this build target */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub kind: Option<BuildTargetEventKind>,
    /** Any additional metadata about what information changed. */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<BuildTargetEventData>,
}
