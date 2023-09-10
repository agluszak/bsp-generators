use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModulesItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    #[serde(default)]
    pub modules: Vec<DependencyModule>,
}
