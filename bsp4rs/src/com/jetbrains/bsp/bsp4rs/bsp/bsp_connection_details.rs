use serde::{Deserialize, Serialize};

/** Structure describing how to start a BSP server and the capabilities it supports. */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
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
