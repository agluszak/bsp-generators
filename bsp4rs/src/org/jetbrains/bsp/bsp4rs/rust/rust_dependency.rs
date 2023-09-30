use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustDependency {
    /// The Package ID of the dependency.
    pub pkg: String,
    /// The name of the dependency's library target.
    /// If this is a renamed dependency, this is the new name.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of dependency kinds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub dep_kinds: Option<Vec<RustDepKindInfo>>,
}
