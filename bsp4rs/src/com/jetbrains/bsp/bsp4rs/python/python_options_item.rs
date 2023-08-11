use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsItem {
    pub target: BuildTargetIdentifier,
    /** Attributes added to the interpreter command
    For example: -E */
    pub interpreter_options: Vec<String>,
}
