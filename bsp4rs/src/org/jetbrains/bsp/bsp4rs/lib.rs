use serde::de::DeserializeOwned;
use serde::{Deserialize, Serialize};

pub mod bsp;
pub mod cancel;
pub mod cargo;
pub mod cpp;
pub mod java;
pub mod jvm;
pub mod maven;
pub mod python;
pub mod rust;
pub mod sbt;
pub mod scala;

use bsp::*;
use cancel::*;
use cargo::*;
use cpp::*;
use java::*;
use jvm::*;
use maven::*;
use python::*;
use rust::*;
use sbt::*;
use scala::*;

pub const PROTOCOL_VERSION: &str = "2.1.0";

pub trait Request {
    type Params: DeserializeOwned + Serialize;
    type Result: DeserializeOwned + Serialize;
    const METHOD: &'static str;
}

pub trait Notification {
    type Params: DeserializeOwned + Serialize;
    const METHOD: &'static str;
}

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OtherData {
    pub data_kind: String,
    pub data: serde_json::Value,
}
