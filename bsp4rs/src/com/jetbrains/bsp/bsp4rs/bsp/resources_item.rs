use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    pub target: BuildTargetIdentifier,
    /** List of resource files. */
    pub resources: Vec<String>,
}
