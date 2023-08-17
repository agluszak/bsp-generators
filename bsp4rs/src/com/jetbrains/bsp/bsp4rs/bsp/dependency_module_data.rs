use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDependencyModuleData {
    Maven(MavenDependencyModule),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyModuleData {
    Named(NamedDependencyModuleData),
    Other(OtherData),
}

impl DependencyModuleData {
    pub fn maven(data: MavenDependencyModule) -> Self {
        DependencyModuleData::Named(NamedDependencyModuleData::Maven(data))
    }
}
