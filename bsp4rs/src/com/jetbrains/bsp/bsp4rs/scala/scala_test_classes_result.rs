use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesResult {
    /** An optional id of the request that triggered this result. */
    #[serde(default)]
    pub items: Vec<ScalaTestClassesItem>,
}
