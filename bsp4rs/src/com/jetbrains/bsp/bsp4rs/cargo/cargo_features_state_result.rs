use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CargoFeaturesStateResult {
    /** The list of Cargo packages with assigned to them target
    identifiers and available features. */
    pub packages_features: PackageFeatures,
}
