use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Attributes added to the interpreter command
    For example: -E */
    #[serde(default)]
    pub interpreter_options: Vec<String>,
}