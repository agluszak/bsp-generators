use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedInitializeBuildParamsData {}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InitializeBuildParamsData {
    Named(NamedInitializeBuildParamsData),
    Other(OtherData),
}

impl InitializeBuildParamsData {}
