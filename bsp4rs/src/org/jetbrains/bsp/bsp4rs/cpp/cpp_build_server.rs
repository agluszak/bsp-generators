use crate::*;

/// The build target cpp options request is sent from the client to the server to
/// query for the list of compiler options necessary to compile in a given list of
/// targets.
#[derive(Debug)]
pub enum BuildTargetCppOptions {}

impl Request for BuildTargetCppOptions {
    type Params = CppOptionsParams;
    type Result = CppOptionsResult;
    const METHOD: &'static str = "buildTarget/cppOptions";
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn build_target_cpp_options_method() {
        assert_eq!(BuildTargetCppOptions::METHOD, "buildTarget/cppOptions");
    }
}
