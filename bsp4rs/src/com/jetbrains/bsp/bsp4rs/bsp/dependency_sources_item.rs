use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DependencySourcesItem {
    pub target: BuildTargetIdentifier,
    /** List of resources containing source files of the
    target's dependencies.
    Can be source files, jar files, zip files, or directories. */
    pub sources: Vec<String>,
}
