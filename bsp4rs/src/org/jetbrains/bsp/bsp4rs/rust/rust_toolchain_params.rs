use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustToolchainParams {
    /** A sequence of build targets for toolchain resolution. */
    #[serde(default)]
    pub targets: Vec<BuildTargetIdentifier>,
}
