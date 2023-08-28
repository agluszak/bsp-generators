use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmMainClass {
    #[serde(default)]
    pub class_name: String,
    #[serde(default)]
    pub arguments: Vec<String>,
}
