use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildParams {
    /** Name of the client */
    pub display_name: String,
    /** The version of the client */
    pub version: String,
    /** The BSP version that the client speaks */
    pub bsp_version: String,
    /** The rootUri of the workspace */
    pub root_uri: URI,
    /** The capabilities of the client */
    pub capabilities: BuildClientCapabilities,
    /** Additional metadata about the client */
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InitializeBuildParamsData>,
}
