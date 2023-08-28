use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavacOptionsResult {
    #[serde(default)]
    pub items: Vec<JavacOptionsItem>,
}
