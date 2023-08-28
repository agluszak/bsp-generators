use serde::{Deserialize, Serialize};

use crate::*;

/** The beginning of a testing unit may be signalled to the client with a
`build/taskStart` notification. When the testing unit is a build target, the
notification's `dataKind` field must be `test-task` and the `data` field must
include a `TestTask` object. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTask {
    #[serde(default)]
    pub target: BuildTargetIdentifier,
}
