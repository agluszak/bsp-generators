use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildParams {
    /** Name of the client */
    #[serde(default)]
    pub display_name: String,
    /** The version of the client */
    #[serde(default)]
    pub version: String,
    /** The BSP version that the client speaks */
    #[serde(default)]
    pub bsp_version: String,
    /** The rootUri of the workspace */
    #[serde(default)]
    pub root_uri: URI,
    /** The capabilities of the client */
    #[serde(default)]
    pub capabilities: BuildClientCapabilities,
    /** Additional metadata about the client */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<InitializeBuildParamsData>,
}
