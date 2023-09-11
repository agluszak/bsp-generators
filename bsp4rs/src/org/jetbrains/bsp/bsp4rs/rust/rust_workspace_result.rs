use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustWorkspaceResult {
    /** Packages of given targets. */
    #[serde(default)]
    pub packages: Vec<RustPackage>,
    /** Dependencies as listed in the package `Cargo.toml`,
    without package resolution or any additional data. */
    #[serde(default)]
    pub raw_dependencies: RustRawDependencies,
    /** Resolved dependencies of the package. Handles renamed dependencies. */
    #[serde(default)]
    pub dependencies: RustDependencies,
    /** A sequence of build targets taken into consideration during build process. */
    #[serde(default)]
    pub resolved_targets: Vec<BuildTargetIdentifier>,
}
