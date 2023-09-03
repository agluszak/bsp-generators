use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDebugSessionParamsData {
    ScalaAttachRemote(ScalaAttachRemote),
    ScalaMainClass(ScalaMainClass),
    ScalaTestSuites(Vec<String>),
    ScalaTestSuitesSelection(ScalaTestSuites),
}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DebugSessionParamsData {
    Named(NamedDebugSessionParamsData),
    Other(OtherData),
}

impl DebugSessionParamsData {
    pub fn scala_attach_remote(data: ScalaAttachRemote) -> Self {
        DebugSessionParamsData::Named(NamedDebugSessionParamsData::ScalaAttachRemote(data))
    }
    pub fn scala_main_class(data: ScalaMainClass) -> Self {
        DebugSessionParamsData::Named(NamedDebugSessionParamsData::ScalaMainClass(data))
    }
    pub fn scala_test_suites(data: Vec<String>) -> Self {
        DebugSessionParamsData::Named(NamedDebugSessionParamsData::ScalaTestSuites(data))
    }
    pub fn scala_test_suites_selection(data: ScalaTestSuites) -> Self {
        DebugSessionParamsData::Named(NamedDebugSessionParamsData::ScalaTestSuitesSelection(data))
    }
}
