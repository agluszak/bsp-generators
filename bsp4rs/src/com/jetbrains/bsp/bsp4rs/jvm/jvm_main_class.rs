use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JvmMainClass {
    pub class_name: String,
    pub arguments: Vec<String>,
}
