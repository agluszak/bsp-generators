use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedInitializeBuildResultData {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializeBuildResultData {
    Named(NamedInitializeBuildResultData),
    Other(OtherData),
}

impl InitializeBuildResultData {}
