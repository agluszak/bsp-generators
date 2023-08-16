use serde::{Deserialize, Serialize};

/** Structure describing how to start a BSP server and the capabilities it supports. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BspConnectionDetails {
    /** The name of the BSP server. */
    pub name: String,
    /** Arguments to pass to the BSP server. */
    pub argv: Vec<String>,
    /** The version of the BSP server. */
    pub version: String,
    /** Supported BSP version. */
    pub bsp_version: String,
    /** The languages supported by the BSP server. */
    pub languages: Vec<String>,
}
