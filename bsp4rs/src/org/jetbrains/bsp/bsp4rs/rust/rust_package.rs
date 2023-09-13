use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::{BTreeMap, BTreeSet};

/// A `crate` is the smallest amount of code that the Rust compiler considers at a time.
/// It can come in one of two forms: a binary crate or a library crate.
/// `Binary crates` are programs you can compile to an executable that you can run,
/// such as a command-line program or a server.
/// Each must have a function called main that defines what happens when the executable runs.
/// `Library crates` don’t have a main function, and they don’t compile to an executable.
/// Instead, they define functionality intended to be shared with multiple projects.
///
/// A `package` is a bundle of one or more crates that provides a set of functionality.
/// It contains a Cargo.toml file that describes how to build those crates.
/// A package can contain many binary crates, but at most only one library crate.
/// However, it must contain at least one crate, whether that’s a library or binary crate.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustPackage {
    /// The package’s unique identifier
    pub id: String,
    /// The package's root path.
    pub root_url: URI,
    /// The name of the package.
    pub name: String,
    /// The version of the package.
    pub version: String,
    /// Defines a reason a package is in a project.
    pub origin: RustPackageOrigin,
    /// Code edition of the package.
    pub edition: RustEdition,
    /// The source ID of the dependency, `null` for the root package and path dependencies.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Corresponds to source files which can be compiled into a crate from this package.
    /// Contains only resolved targets without conflicts.
    pub resolved_targets: Vec<RustBuildTarget>,
    /// Same as `targets`, but contains all targets from this package.
    /// `targets` should be the subset of `allTargets`.
    pub all_targets: Vec<RustBuildTarget>,
    /// Set of features defined for the package.
    /// Each feature maps to an array of features or dependencies it enables.
    /// The entry named "default" defines which features are enabled by default.
    pub features: FeatureDependencyGraph,
    /// Array of features enabled on this package.
    pub enabled_features: BTreeSet<Feature>,
    /// Conditional compilation flags that can be set based on certain conditions.
    /// They can be used to enable or disable certain sections of code during the build process.
    /// `cfgs` in Rust can take one of two forms: "cfg1" or "cfg2=\"string\"".
    /// The `cfg` is split by '=' delimiter and the first half becomes key and
    /// the second is aggregated to the value in `RustCfgOptions`.
    /// For "cfg1" the value is empty.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub cfg_options: RustCfgOptions,
    /// Environment variables for the package.
    #[serde(default, skip_serializing_if = "BTreeMap::is_empty")]
    pub env: EnvironmentVariables,
    /// An absolute path which is used as a value of `OUT_DIR` environmental
    /// variable when compiling current package.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub out_dir_url: Option<URI>,
    /// File path to compiled output of a procedural macro crate.
    /// Procedural macros are macros that generate code at compile time.
    /// Contains files with file extensions: `.dll`, `.so` or `.dylib`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub proc_macro_artifact: Option<String>,
}
