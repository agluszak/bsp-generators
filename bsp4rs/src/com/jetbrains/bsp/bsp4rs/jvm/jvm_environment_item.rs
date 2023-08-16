use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmEnvironmentItem {
    pub target: BuildTargetIdentifier,
    pub classpath: Vec<String>,
    pub jvm_options: Vec<String>,
    pub working_directory: String,
    pub environment_variables: HashMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_classes: Vec<JvmMainClass>,
}
