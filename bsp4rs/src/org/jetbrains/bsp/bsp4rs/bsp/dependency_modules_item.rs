use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModulesItem {
    pub target: BuildTargetIdentifier,
    pub modules: Vec<DependencyModule>,
}

#[cfg(test)]
mod tests {}
