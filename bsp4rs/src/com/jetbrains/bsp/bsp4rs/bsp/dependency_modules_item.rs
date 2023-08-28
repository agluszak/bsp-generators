use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModulesItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    #[serde(default)]
    pub modules: Vec<DependencyModule>,
}
