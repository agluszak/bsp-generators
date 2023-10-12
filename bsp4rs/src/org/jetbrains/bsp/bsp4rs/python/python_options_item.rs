use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsItem {
    pub target: BuildTargetIdentifier,
    /// Attributes added to the interpreter command
    /// For example: -E
    pub interpreter_options: Vec<String>,
}

#[cfg(test)]
mod tests {}
