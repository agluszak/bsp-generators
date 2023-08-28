use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesResult {
    /** An optional id of the request that triggered this result. */
    #[serde(default)]
    pub items: Vec<ScalaTestClassesItem>,
}
