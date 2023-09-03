use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JvmEnvironmentItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    #[serde(default)]
    pub classpath: Vec<String>,
    #[serde(default)]
    pub jvm_options: Vec<String>,
    #[serde(default)]
    pub working_directory: String,
    #[serde(default)]
    pub environment_variables: BTreeMap<String, String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub main_classes: Vec<JvmMainClass>,
}
