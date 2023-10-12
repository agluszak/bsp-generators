use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustRawDependency {
    /// The name of the dependency.
    pub name: String,
    /// Name to which this dependency is renamed when declared in Cargo.toml.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rename: Option<String>,
    /// The dependency kind.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub kind: Option<RustDepKind>,
    /// The target platform for the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
    /// Indicates whether this is an optional dependency.
    pub optional: bool,
    /// Indicates whether default features are enabled.
    pub uses_default_features: bool,
    /// A sequence of enabled features.
    pub features: BTreeSet<Feature>,
}

#[cfg(test)]
mod tests {}
