use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCargoFeaturesParams {
    /** Package ID for which new features state will be set. */
    #[serde(default)]
    pub package_id: String,
    /** The list of features to be set as a new state. */
    #[serde(default)]
    pub features: Vec<String>,
}
