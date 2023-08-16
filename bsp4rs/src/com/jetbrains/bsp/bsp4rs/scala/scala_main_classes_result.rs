use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesResult {
    pub items: Vec<ScalaMainClassesItem>,
    /** An optional id of the request that triggered this result. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
