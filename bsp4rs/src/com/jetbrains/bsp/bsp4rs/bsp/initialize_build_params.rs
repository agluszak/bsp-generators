use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildParams {
    /** Name of the client */
    pub display_name: String,
    /** The version of the client */
    pub version: String,
    /** The BSP version that the client speaks */
    pub bsp_version: String,
    /** The rootUri of the workspace */
    pub root_uri: String,
    /** The capabilities of the client */
    pub capabilities: BuildClientCapabilities,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** Additional metadata about the client */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
