use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDebugSessionParamsData {
    ScalaAttachRemote(ScalaAttachRemote),
    ScalaMainClass(ScalaMainClass),
    ScalaTestSuites(Vec<String>),
    ScalaTestSuitesSelection(ScalaTestSuites),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DebugSessionParamsData {
    Named(NamedDebugSessionParamsData),
    Other(OtherData),
}

impl DebugSessionParamsData {
    pub fn scala_attach_remote(data: ScalaAttachRemote) -> Self {
        Self::Named(NamedDebugSessionParamsData::ScalaAttachRemote(data))
    }
    pub fn scala_main_class(data: ScalaMainClass) -> Self {
        Self::Named(NamedDebugSessionParamsData::ScalaMainClass(data))
    }
    pub fn scala_test_suites(data: Vec<String>) -> Self {
        Self::Named(NamedDebugSessionParamsData::ScalaTestSuites(data))
    }
    pub fn scala_test_suites_selection(data: ScalaTestSuites) -> Self {
        Self::Named(NamedDebugSessionParamsData::ScalaTestSuitesSelection(data))
    }
}
