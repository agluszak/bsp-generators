use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTestResultData {}

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestResultData {
    Named(NamedTestResultData),
    Other(OtherData),
}

impl TestResultData {}
