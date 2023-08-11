use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum DebugSessionParamsDataKind {
    ScalaAttachRemote,
    ScalaMainClass,
    ScalaTestSuites,
    ScalaTestSuitesSelection,
}
