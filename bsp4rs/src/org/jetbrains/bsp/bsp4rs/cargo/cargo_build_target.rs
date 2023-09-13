use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

/** `CargoBuildTarget` is a basic data structure that contains
cargo-specific metadata. */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoBuildTarget {
    pub edition: RustEdition,
    pub required_features: BTreeSet<Feature>,
}
