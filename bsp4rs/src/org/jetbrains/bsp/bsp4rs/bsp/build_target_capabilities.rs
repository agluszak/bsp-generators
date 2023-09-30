use serde::{Deserialize, Serialize};

/// Clients can use these capabilities to notify users what BSP endpoints can and
/// cannot be used and why.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetCapabilities {
    /// This target can be compiled by the BSP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_compile: Option<bool>,
    /// This target can be tested by the BSP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_test: Option<bool>,
    /// This target can be run by the BSP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_run: Option<bool>,
    /// This target can be debugged by the BSP server.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub can_debug: Option<bool>,
}
