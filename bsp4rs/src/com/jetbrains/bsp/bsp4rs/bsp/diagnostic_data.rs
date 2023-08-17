use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDiagnosticData {
    Scala(ScalaDiagnostic),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticData {
    Named(NamedDiagnosticData),
    Other(OtherData),
}

impl DiagnosticData {
    pub fn scala(data: ScalaDiagnostic) -> Self {
        DiagnosticData::Named(NamedDiagnosticData::Scala(data))
    }
}
