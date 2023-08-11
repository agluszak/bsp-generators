use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesResult {
    /** An optional id of the request that triggered this result. */
    pub items: Vec<ScalaTestClassesItem>,
}
