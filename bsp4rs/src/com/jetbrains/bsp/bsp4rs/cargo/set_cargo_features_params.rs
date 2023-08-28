use serde::{Deserialize, Serialize};

use std::collections::BTreeSet;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCargoFeaturesParams {
    /** Package ID for which new features state will be set. */
    #[serde(default)]
    pub package_id: String,
    /** The list of features to be set as a new state. */
    #[serde(default)]
    pub features: BTreeSet<String>,
}
