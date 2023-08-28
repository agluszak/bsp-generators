use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

/** `CargoBuildTarget` is a basic data structure that contains
cargo-specific metadata. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoBuildTarget {
    #[serde(default)]
    pub edition: Edition,
    #[serde(default)]
    pub required_features: BTreeSet<Feature>,
}
