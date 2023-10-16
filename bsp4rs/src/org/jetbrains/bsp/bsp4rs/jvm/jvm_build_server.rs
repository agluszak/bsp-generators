use crate::*;

/// The JVM test environment request is sent from the client to the server in order to
/// gather information required to launch a Java process. This is useful when the
/// client wants to control the Java process execution, for example to enable custom
/// Java agents or launch a custom main class during unit testing or debugging
///
/// The data provided by this endpoint may change between compilations, so it should
/// not be cached in any form. The client should ask for it right before test execution,
/// after all the targets are compiled.
#[derive(Debug)]
pub enum BuildTargetJvmTestEnvironment {}

impl Request for BuildTargetJvmTestEnvironment {
    type Params = JvmTestEnvironmentParams;
    type Result = JvmTestEnvironmentResult;
    const METHOD: &'static str = "buildTarget/jvmTestEnvironment";
}

/// Similar to `buildTarget/jvmTestEnvironment`, but returns environment
/// that should be used for regular exection of main classes, not for testing
#[derive(Debug)]
pub enum BuildTargetJvmRunEnvironment {}

impl Request for BuildTargetJvmRunEnvironment {
    type Params = JvmRunEnvironmentParams;
    type Result = JvmRunEnvironmentResult;
    const METHOD: &'static str = "buildTarget/jvmRunEnvironment";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_target_jvm_test_environment_method() {
        assert_eq!(
            BuildTargetJvmTestEnvironment::METHOD,
            "buildTarget/jvmTestEnvironment"
        );
    }

    #[test]
    fn build_target_jvm_run_environment_method() {
        assert_eq!(
            BuildTargetJvmRunEnvironment::METHOD,
            "buildTarget/jvmRunEnvironment"
        );
    }
}
