use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsResult {
    /// The list of options for each target.
    pub items: Vec<CppOptionsItem>,
}

#[cfg(test)]
mod tests {}
