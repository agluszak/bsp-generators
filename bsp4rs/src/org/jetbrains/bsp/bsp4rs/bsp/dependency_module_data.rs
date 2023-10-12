use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDependencyModuleData {
    Maven(MavenDependencyModule),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DependencyModuleData {
    Named(NamedDependencyModuleData),
    Other(OtherData),
}

impl DependencyModuleData {
    pub fn maven(data: MavenDependencyModule) -> Self {
        Self::Named(NamedDependencyModuleData::Maven(data))
    }
}

#[cfg(test)]
mod tests {}
