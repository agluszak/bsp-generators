use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModulesItem {
    pub target: BuildTargetIdentifier,
    pub modules: Vec<DependencyModule>,
}
