use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsParams {
    /** The targets for which the options are requested. */
    pub targets: Vec<BuildTargetIdentifier>,
}
