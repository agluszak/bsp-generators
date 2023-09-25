use crate::*;

#[derive(Debug)]
pub enum WorkspaceLibraries {}

impl Request for WorkspaceLibraries {
    type Params = ();
    type Result = WorkspaceLibrariesResult;
    const METHOD: &'static str = "workspace/libraries";
}
