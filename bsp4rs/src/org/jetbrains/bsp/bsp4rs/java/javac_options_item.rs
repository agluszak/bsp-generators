use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavacOptionsItem {
    pub target: BuildTargetIdentifier,
    /// Additional arguments to the compiler.
    /// For example, -deprecation.
    pub options: Vec<String>,
    /// The dependency classpath for this target, must be
    /// identical to what is passed as arguments to
    /// the -classpath flag in the command line interface
    /// of javac.
    pub classpath: Vec<String>,
    /// The output directory for classfiles produced by this target
    pub class_directory: String,
}

#[cfg(test)]
mod tests {}
