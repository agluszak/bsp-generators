use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildResult {
    /** Name of the server */
    pub display_name: String,
    /** The version of the server */
    pub version: String,
    /** The BSP version that the server speaks */
    pub bsp_version: String,
    /** The capabilities of the build server */
    pub capabilities: BuildServerCapabilities,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** Additional metadata about the server */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
