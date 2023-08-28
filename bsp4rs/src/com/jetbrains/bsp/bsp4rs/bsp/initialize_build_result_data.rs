use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedInitializeBuildResultData {}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializeBuildResultData {
    Named(NamedInitializeBuildResultData),
    Other(OtherData),
}

impl InitializeBuildResultData {}
