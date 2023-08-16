use serde::{Deserialize, Serialize};

use crate::*;

/** A workspace edit represents changes to many resources managed in the workspace. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaWorkspaceEdit {
    pub changes: Vec<ScalaTextEdit>,
}
