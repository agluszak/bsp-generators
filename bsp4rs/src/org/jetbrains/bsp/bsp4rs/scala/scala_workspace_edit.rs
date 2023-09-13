use serde::{Deserialize, Serialize};

use crate::*;

/** A workspace edit represents changes to many resources managed in the workspace. */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaWorkspaceEdit {
    pub changes: Vec<ScalaTextEdit>,
}
