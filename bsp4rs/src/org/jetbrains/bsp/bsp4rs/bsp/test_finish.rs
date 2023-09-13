use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestFinish {
    /** Name or description of the test. */
    pub display_name: String,
    /** Information about completion of the test, for example an error message. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /** Completion status of the test. */
    pub status: TestStatus,
    /** Source location of the test, as LSP location. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /** Optionally, structured metadata about the test completion.
    For example: stack traces, expected/actual values. */
    #[serde(flatten, default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TestFinishData>,
}
