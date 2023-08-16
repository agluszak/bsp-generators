use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageFeatures {
    /** The Cargo package identifier. */
    pub package_id: String,
    /** The list of build target identifiers assigned to the Cargo package. */
    pub targets: Vec<BuildTargetIdentifier>,
    /** The list of available features for the Cargo package. */
    pub available_features: Vec<String>,
    /** The list of enabled features for the Cargo package. */
    pub enabled_features: Vec<String>,
}
