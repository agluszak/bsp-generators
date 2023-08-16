use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanCacheResult {
    /** Optional message to display to the user. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /** Indicates whether the clean cache request was performed or not. */
    pub cleaned: bool,
}
