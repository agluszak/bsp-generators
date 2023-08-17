use serde::{Deserialize, Serialize};

/** `CargoBuildTarget` is a basic data structure that contains
cargo-specific metadata. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoBuildTarget {
    pub edition: String,
    pub required_features: Vec<String>,
}
