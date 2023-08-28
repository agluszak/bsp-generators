use serde::{Deserialize, Serialize};

/** Structure describing how to start a BSP server and the capabilities it supports. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BspConnectionDetails {
    /** The name of the BSP server. */
    #[serde(default)]
    pub name: String,
    /** Arguments to pass to the BSP server. */
    #[serde(default)]
    pub argv: Vec<String>,
    /** The version of the BSP server. */
    #[serde(default)]
    pub version: String,
    /** Supported BSP version. */
    #[serde(default)]
    pub bsp_version: String,
    /** The languages supported by the BSP server. */
    #[serde(default)]
    pub languages: Vec<String>,
}
