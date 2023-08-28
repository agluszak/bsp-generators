use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedCompileResultData {}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CompileResultData {
    Named(NamedCompileResultData),
    Other(OtherData),
}

impl CompileResultData {}
