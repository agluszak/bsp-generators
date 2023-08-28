use serde::{Deserialize, Serialize};

use crate::*;

/** A unique identifier for a target, can use any URI-compatible encoding as long as it is unique within the workspace.
Clients should not infer metadata out of the URI structure such as the path or query parameters, use `BuildTarget` instead. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetIdentifier {
    /** The target’s Uri */
    #[serde(default)]
    pub uri: URI,
}
