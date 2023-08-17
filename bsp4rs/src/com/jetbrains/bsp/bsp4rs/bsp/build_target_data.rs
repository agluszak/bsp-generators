use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedBuildTargetData {
    Cpp(CppBuildTarget),
    Jvm(JvmBuildTarget),
    Python(PythonBuildTarget),
    Sbt(SbtBuildTarget),
    Scala(ScalaBuildTarget),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTargetData {
    Named(NamedBuildTargetData),
    Other(OtherData),
}

impl BuildTargetData {
    pub fn cpp(data: CppBuildTarget) -> Self {
        BuildTargetData::Named(NamedBuildTargetData::Cpp(data))
    }
    pub fn jvm(data: JvmBuildTarget) -> Self {
        BuildTargetData::Named(NamedBuildTargetData::Jvm(data))
    }
    pub fn python(data: PythonBuildTarget) -> Self {
        BuildTargetData::Named(NamedBuildTargetData::Python(data))
    }
    pub fn sbt(data: SbtBuildTarget) -> Self {
        BuildTargetData::Named(NamedBuildTargetData::Sbt(data))
    }
    pub fn scala(data: ScalaBuildTarget) -> Self {
        BuildTargetData::Named(NamedBuildTargetData::Scala(data))
    }
}
