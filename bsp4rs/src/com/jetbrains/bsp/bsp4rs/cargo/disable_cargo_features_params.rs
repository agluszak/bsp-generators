use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DisableCargoFeaturesParams {
    /** Package ID to disable features for. */
    pub package_id: String,
    /** The list of features to disable. */
    pub features: Vec<String>,
}
