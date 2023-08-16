use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionAddress {
    /** The Debug Adapter Protocol server's connection uri */
    pub uri: String,
}
