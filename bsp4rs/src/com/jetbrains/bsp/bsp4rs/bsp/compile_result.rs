use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileResult {
    /** An optional request id to know the origin of this report. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /** A status code for the execution. */
    #[serde(default)]
    pub status_code: StatusCode,
    /** A field containing language-specific information, like products
    of compilation or compiler-specific metadata the client needs to know. */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<CompileResultData>,
}
