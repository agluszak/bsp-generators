use serde::{Deserialize, Serialize};

/** A unique identifier for a target, can use any URI-compatible encoding as long as it is unique within the workspace.
Clients should not infer metadata out of the URI structure such as the path or query parameters, use `BuildTarget` instead. */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetIdentifier {
    /** The target’s Uri */
    pub uri: String,
}