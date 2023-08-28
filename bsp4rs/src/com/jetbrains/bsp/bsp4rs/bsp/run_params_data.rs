use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedRunParamsData {
    ScalaMainClass(ScalaMainClass),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunParamsData {
    Named(NamedRunParamsData),
    Other(OtherData),
}

impl RunParamsData {
    pub fn scala_main_class(data: ScalaMainClass) -> Self {
        RunParamsData::Named(NamedRunParamsData::ScalaMainClass(data))
    }
}
