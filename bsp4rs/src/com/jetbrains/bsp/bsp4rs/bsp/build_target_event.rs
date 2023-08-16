use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetEvent {
    /** The identifier for the changed build target */
    pub target: BuildTargetIdentifier,
    /** The kind of change for this build target */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<BuildTargetEventKind>,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** Any additional metadata about what information changed. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
