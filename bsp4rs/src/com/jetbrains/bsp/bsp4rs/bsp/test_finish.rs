use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TestFinish {
    /** Name or description of the test. */
    pub display_name: String,
    /** Information about completion of the test, for example an error message. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /** Completion status of the test. */
    pub status: TestStatus,
    /** Source location of the test, as LSP location. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** Optionally, structured metadata about the test completion.
    For example: stack traces, expected/actual values. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
