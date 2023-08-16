use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsResult {
    /** The list of options for each target. */
    pub items: Vec<CppOptionsItem>,
}
