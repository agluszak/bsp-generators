use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    pub target: BuildTargetIdentifier,
    /// List of resource files.
    pub resources: Vec<URI>,
}

#[cfg(test)]
mod tests {}
