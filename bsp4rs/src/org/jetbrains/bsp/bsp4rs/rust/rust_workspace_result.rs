use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustWorkspaceResult {
    /** Packages of given targets. */
    pub packages: Vec<RustPackage>,
    /** Dependencies as listed in the package `Cargo.toml`,
    without package resolution or any additional data. */
    pub raw_dependencies: RustRawDependencies,
    /** Resolved dependencies of the package. Handles renamed dependencies. */
    pub dependencies: RustDependencies,
    /** A sequence of build targets taken into consideration during build process. */
    pub resolved_targets: Vec<BuildTargetIdentifier>,
}
