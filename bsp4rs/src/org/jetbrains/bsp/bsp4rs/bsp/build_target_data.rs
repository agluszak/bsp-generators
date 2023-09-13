use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedBuildTargetData {
    Cargo(CargoBuildTarget),
    Cpp(CppBuildTarget),
    Jvm(JvmBuildTarget),
    Python(PythonBuildTarget),
    Rust(RustBuildTarget),
    Sbt(SbtBuildTarget),
    Scala(ScalaBuildTarget),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTargetData {
    Named(NamedBuildTargetData),
    Other(OtherData),
}

impl BuildTargetData {
    pub fn cargo(data: CargoBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Cargo(data))
    }
    pub fn cpp(data: CppBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Cpp(data))
    }
    pub fn jvm(data: JvmBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Jvm(data))
    }
    pub fn python(data: PythonBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Python(data))
    }
    pub fn rust(data: RustBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Rust(data))
    }
    pub fn sbt(data: SbtBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Sbt(data))
    }
    pub fn scala(data: ScalaBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Scala(data))
    }
}
