use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Attributes added to the interpreter command
    For example: -E */
    #[serde(default)]
    pub interpreter_options: Vec<String>,
}
