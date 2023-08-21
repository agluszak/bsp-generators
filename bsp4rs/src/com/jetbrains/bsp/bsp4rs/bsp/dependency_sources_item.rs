use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencySourcesItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** List of resources containing source files of the
    target's dependencies.
    Can be source files, jar files, zip files, or directories. */
    #[serde(default)]
    pub sources: Vec<URI>,
}
