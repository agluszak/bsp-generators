use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTestParamsData {
    ScalaTest(ScalaTestParams),
}

#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestParamsData {
    Named(NamedTestParamsData),
    Other(OtherData),
}

impl TestParamsData {
    pub fn scala_test(data: ScalaTestParams) -> Self {
        TestParamsData::Named(NamedTestParamsData::ScalaTest(data))
    }
}
