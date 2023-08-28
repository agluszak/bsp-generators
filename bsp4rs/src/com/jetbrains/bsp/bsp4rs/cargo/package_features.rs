use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::{BTreeMap, BTreeSet};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageFeatures {
    /** The Cargo package identifier. */
    #[serde(default)]
    pub package_id: String,
    /** The list of build target identifiers assigned to the Cargo package. */
    #[serde(default)]
    pub targets: Vec<BuildTargetIdentifier>,
    /** The list of available features for the Cargo package. */
    #[serde(default)]
    pub available_features: BTreeMap<Feature, BTreeSet<Feature>>,
    /** The list of enabled features for the Cargo package. */
    #[serde(default)]
    pub enabled_features: BTreeSet<Feature>,
}
