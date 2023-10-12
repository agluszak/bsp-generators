use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InitializeBuildResult {
    /// Name of the server
    pub display_name: String,
    /// The version of the server
    pub version: String,
    /// The BSP version that the server speaks
    pub bsp_version: String,
    /// The capabilities of the build server
    pub capabilities: BuildServerCapabilities,
    /// Additional metadata about the server
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<InitializeBuildResultData>,
}

#[cfg(test)]
mod tests {}
