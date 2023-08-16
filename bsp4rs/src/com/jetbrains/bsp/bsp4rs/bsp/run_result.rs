use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    /** An optional request id to know the origin of this report. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    /** A status code for the execution. */
    pub status_code: StatusCode,
}
