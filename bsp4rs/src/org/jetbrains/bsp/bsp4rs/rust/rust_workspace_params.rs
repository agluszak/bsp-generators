use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustWorkspaceParams {
    /** A sequence of build targets for workspace resolution. */
    #[serde(default)]
    pub targets: Vec<BuildTargetIdentifier>,
}