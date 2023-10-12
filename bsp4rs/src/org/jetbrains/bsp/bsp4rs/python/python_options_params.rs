use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsParams {
    pub targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {}
