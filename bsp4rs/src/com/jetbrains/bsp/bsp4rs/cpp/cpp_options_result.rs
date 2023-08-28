use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsResult {
    /** The list of options for each target. */
    #[serde(default)]
    pub items: Vec<CppOptionsItem>,
}
