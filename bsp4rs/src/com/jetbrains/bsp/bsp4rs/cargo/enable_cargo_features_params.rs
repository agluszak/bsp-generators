use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct EnableCargoFeaturesParams {
    /** Package ID to enable features for. */
    pub package_id: String,
    /** The list of features to enable. */
    pub features: Vec<String>,
}
