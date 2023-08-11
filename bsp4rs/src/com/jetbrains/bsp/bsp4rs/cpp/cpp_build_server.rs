use crate::*;

#[derive(Debug)]
pub enum BuildTargetCppOptions {}

impl Request for BuildTargetCppOptions {
    type Params = CppOptionsParams;
    type Result = CppOptionsResult;
    const METHOD: &'static str = "buildTarget/cppOptions";
}
