use crate::*;

#[derive(Debug)]
pub enum BuildTargetJvmTestEnvironment {}

impl Request for BuildTargetJvmTestEnvironment {
    type Params = JvmTestEnvironmentParams;
    type Result = JvmTestEnvironmentResult;
    const METHOD: &'static str = "buildTarget/jvmTestEnvironment";
}

#[derive(Debug)]
pub enum BuildTargetJvmRunEnvironment {}

impl Request for BuildTargetJvmRunEnvironment {
    type Params = JvmRunEnvironmentParams;
    type Result = JvmRunEnvironmentResult;
    const METHOD: &'static str = "buildTarget/jvmRunEnvironment";
}
