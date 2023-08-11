use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CompileParams {
    /** A sequence of build targets to compile. */
    pub targets: Vec<BuildTargetIdentifier>,
    /** A unique identifier generated by the client to identify this request.
    The server may include this id in triggered notifications or responses. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    /** Optional arguments to the compilation process. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
}
