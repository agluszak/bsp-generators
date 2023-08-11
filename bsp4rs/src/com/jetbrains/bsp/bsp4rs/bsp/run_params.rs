use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct RunParams {
    /** The build target to run. */
    pub target: BuildTargetIdentifier,
    /** A unique identifier generated by the client to identify this request.
    The server may include this id in triggered notifications or responses. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    /** Optional arguments to the executed application. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub arguments: Vec<String>,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** Language-specific metadata for this execution.
    See ScalaMainClass as an example. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}