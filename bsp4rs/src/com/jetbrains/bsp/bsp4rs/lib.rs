use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod bsp;
pub mod cargo;
pub mod cpp;
pub mod java;
pub mod jvm;
pub mod maven;
pub mod python;
pub mod sbt;
pub mod scala;

use bsp::*;
use cargo::*;
use cpp::*;
use java::*;
use jvm::*;
use maven::*;
use python::*;
use sbt::*;
use scala::*;

pub trait Request {
    type Params: DeserializeOwned + Serialize;
    type Result: DeserializeOwned + Serialize;
    const METHOD: &'static str;
}

pub trait Notification {
    type Params: DeserializeOwned + Serialize;
    const METHOD: &'static str;
}

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherData {
    #[serde(default)]
    pub data_kind: String,
    #[serde(default, flatten)]
    pub data: serde_json::Value,
}
