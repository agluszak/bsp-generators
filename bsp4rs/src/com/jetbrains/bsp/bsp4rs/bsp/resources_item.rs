use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    pub target: BuildTargetIdentifier,
    /** List of resource files. */
    pub resources: Vec<String>,
}
