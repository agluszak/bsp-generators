use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageFeatures {
    /// The Cargo package identifier.
    pub package_id: String,
    /// The list of build target identifiers assigned to the Cargo package.
    pub targets: Vec<BuildTargetIdentifier>,
    /// The list of available features for the Cargo package.
    pub available_features: FeatureDependencyGraph,
    /// The list of enabled features for the Cargo package.
    pub enabled_features: BTreeSet<Feature>,
}

#[cfg(test)]
mod tests {}
