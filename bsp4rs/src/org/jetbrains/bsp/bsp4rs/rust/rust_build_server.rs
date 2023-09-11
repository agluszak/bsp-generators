use crate::*;

#[derive(Debug)]
pub enum RustWorkspace {}

impl Request for RustWorkspace {
    type Params = RustWorkspaceParams;
    type Result = RustWorkspaceResult;
    const METHOD: &'static str = "buildTarget/rustWorkspace";
}

#[derive(Debug)]
pub enum RustToolchain {}

impl Request for RustToolchain {
    type Params = RustToolchainParams;
    type Result = RustToolchainResult;
    const METHOD: &'static str = "buildTarget/rustToolchain";
}
