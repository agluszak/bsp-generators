use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoFeaturesStateResult {
    /** The list of Cargo packages with assigned to them target
    identifiers and available features. */
    #[serde(default)]
    pub packages_features: Vec<PackageFeatures>,
}
