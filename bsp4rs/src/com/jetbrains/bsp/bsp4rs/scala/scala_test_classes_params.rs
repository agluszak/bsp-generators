use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesParams {
    #[serde(default)]
    pub targets: Vec<BuildTargetIdentifier>,
    /** An optional number uniquely identifying a client request. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
