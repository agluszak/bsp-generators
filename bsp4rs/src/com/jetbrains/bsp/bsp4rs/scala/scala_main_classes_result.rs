use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesResult {
    #[serde(default)]
    pub items: Vec<ScalaMainClassesItem>,
    /** An optional id of the request that triggered this result. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
