use crate::*;

#[derive(Debug)]
pub enum RustWorkspace {}

/// The Rust workspace request is sent from the client to the server to query for
/// the information about project's workspace for the given list of build targets.
///
/// The request is essential to connect and work with `intellij-rust` plugin.
///
/// The request may take a long time, as it may require building a project to some extent
/// (for example with `cargo check` command).
impl Request for RustWorkspace {
    type Params = RustWorkspaceParams;
    type Result = RustWorkspaceResult;
    const METHOD: &'static str = "buildTarget/rustWorkspace";
}

#[derive(Debug)]
pub enum RustToolchain {}

/// The Rust toolchain request is sent from the client to the server to query for
/// the information about project's toolchain for the given list of build targets.
///
/// The request is essential to connect and work with `intellij-rust` plugin.
impl Request for RustToolchain {
    type Params = RustToolchainParams;
    type Result = RustToolchainResult;
    const METHOD: &'static str = "buildTarget/rustToolchain";
}
