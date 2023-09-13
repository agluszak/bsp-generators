use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustDepKindInfo {
    /// The dependency kind.
    pub kind: RustDepKind,
    /// The target platform for the dependency.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}
