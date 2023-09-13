use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTestParamsData {
    ScalaTest(ScalaTestParams),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestParamsData {
    Named(NamedTestParamsData),
    Other(OtherData),
}

impl TestParamsData {
    pub fn scala_test(data: ScalaTestParams) -> Self {
        Self::Named(NamedTestParamsData::ScalaTest(data))
    }
}
