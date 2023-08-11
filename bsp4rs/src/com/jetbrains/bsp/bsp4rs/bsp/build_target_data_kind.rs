use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTargetDataKind {
    Cpp,
    Jvm,
    Python,
    Sbt,
    Scala,
}
