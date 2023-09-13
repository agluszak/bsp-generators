use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

/// This structure is embedded in the `data?: BuildTargetData` field, when the
/// `dataKind` field contains "rust".
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustBuildTarget {
    /// The name of the target.
    pub name: String,
    /// Path to the root module of the crate.
    pub crate_root_url: URI,
    /// A target's kind.
    pub kind: RustTargetKind,
    /// Type of output that is produced by a crate during the build process.
    /// The crate type determines how the source code is compiled.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub crate_types: Vec<RustCrateType>,
    /// The Rust edition of the target.
    pub edition: RustEdition,
    /// Whether or not this target has doc tests enabled, and
    /// the target is compatible with doc testing.
    pub doctest: bool,
    /// A sequence of required features.
    #[serde(default, skip_serializing_if = "BTreeSet::is_empty")]
    pub required_features: BTreeSet<Feature>,
}
