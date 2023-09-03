use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenDependencyModuleArtifact {
    /** Path to jar */
    #[serde(default)]
    pub uri: URI,
    /** Empty or `sources`|`docs` */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub classifier: Option<String>,
}
