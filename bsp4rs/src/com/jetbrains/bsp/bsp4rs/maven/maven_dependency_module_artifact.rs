use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenDependencyModuleArtifact {
    /** Path to jar */
    pub uri: String,
    /** Empty or `sources`|`docs` */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub classifier: Option<String>,
}
