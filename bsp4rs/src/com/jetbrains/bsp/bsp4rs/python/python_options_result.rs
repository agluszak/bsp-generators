use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsResult {
    pub items: Vec<PythonOptionsItem>,
}
