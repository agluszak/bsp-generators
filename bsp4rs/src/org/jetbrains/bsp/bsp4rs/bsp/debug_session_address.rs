use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionAddress {
    /// The Debug Adapter Protocol server's connection uri
    pub uri: URI,
}

#[cfg(test)]
mod tests {}
