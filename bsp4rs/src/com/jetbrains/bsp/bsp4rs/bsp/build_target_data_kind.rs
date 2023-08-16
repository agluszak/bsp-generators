use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum BuildTargetDataKind {
    #[default]
    Cpp,
    Jvm,
    Python,
    Sbt,
    Scala,
}
