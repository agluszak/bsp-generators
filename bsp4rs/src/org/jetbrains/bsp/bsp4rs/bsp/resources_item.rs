use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** List of resource files. */
    #[serde(default)]
    pub resources: Vec<URI>,
}
