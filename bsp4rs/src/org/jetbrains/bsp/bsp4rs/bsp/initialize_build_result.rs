use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildResult {
    /** Name of the server */
    #[serde(default)]
    pub display_name: String,
    /** The version of the server */
    #[serde(default)]
    pub version: String,
    /** The BSP version that the server speaks */
    #[serde(default)]
    pub bsp_version: String,
    /** The capabilities of the build server */
    #[serde(default)]
    pub capabilities: BuildServerCapabilities,
    /** Additional metadata about the server */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<InitializeBuildResultData>,
}
