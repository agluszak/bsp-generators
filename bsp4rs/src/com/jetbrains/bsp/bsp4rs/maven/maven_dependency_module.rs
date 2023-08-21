use serde::{Deserialize, Serialize};

use crate::*;

/** `MavenDependencyModule` is a basic data structure that contains maven-like
metadata. This metadata is embedded in the `data: Option[Json]` field of the `DependencyModule` definition, when the `dataKind` field contains "maven". */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct MavenDependencyModule {
    #[serde(default)]
    pub organization: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub version: String,
    /** List of module's artifacts with different classifiers.
    For example: [
      {uri = "../scala-library-2.13.5.jar"},
      {uri = "../scala-library-2.13.5-sources.jar", classifier = "sources"}
    ] */
    #[serde(default)]
    pub artifacts: Vec<MavenDependencyModuleArtifact>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scope: Option<String>,
}
