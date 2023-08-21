use serde::{Deserialize, Serialize};

use crate::*;

/** The beginning of a compilation unit may be signalled to the client with a
`build/taskStart` notification. When the compilation unit is a build target, the
notification's `dataKind` field must be "compile-task" and the `data` field must
include a `CompileTask` object: */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileTask {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
}
