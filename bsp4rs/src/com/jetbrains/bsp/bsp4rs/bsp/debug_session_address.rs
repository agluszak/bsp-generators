use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionAddress {
    /** The Debug Adapter Protocol server's connection uri */
    #[serde(default)]
    pub uri: URI,
}
