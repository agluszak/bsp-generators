use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDiagnosticData {
    Scala(ScalaDiagnostic),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
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
