use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** List of resource files. */
    #[serde(default)]
    pub resources: Vec<URI>,
}
