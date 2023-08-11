use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsParams {
    /** The targets for which the options are requested. */
    pub targets: Vec<BuildTargetIdentifier>,
}
