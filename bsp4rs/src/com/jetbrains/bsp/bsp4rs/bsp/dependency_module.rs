use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModule {
    /** Module name */
    #[serde(default)]
    pub name: String,
    /** Module version */
    #[serde(default)]
    pub version: String,
    /** Language-specific metadata about this module.
    See MavenDependencyModule as an example. */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<DependencyModuleData>,
}
