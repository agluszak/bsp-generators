use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModule {
    /** Module name */
    pub name: String,
    /** Module version */
    pub version: String,
    /** Language-specific metadata about this module.
    See MavenDependencyModule as an example. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<DependencyModuleData>,
}
