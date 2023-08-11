use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesParams {
    pub targets: Vec<BuildTargetIdentifier>,
    /** An optional number uniquely identifying a client request. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
