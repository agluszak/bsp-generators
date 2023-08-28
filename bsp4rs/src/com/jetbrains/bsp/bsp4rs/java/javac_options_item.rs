use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavacOptionsItem {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Additional arguments to the compiler.
    For example, -deprecation. */
    #[serde(default)]
    pub options: Vec<String>,
    /** The dependency classpath for this target, must be
    identical to what is passed as arguments to
    the -classpath flag in the command line interface
    of javac. */
    #[serde(default)]
    pub classpath: Vec<String>,
    /** The output directory for classfiles produced by this target */
    #[serde(default)]
    pub class_directory: String,
}
