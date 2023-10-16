use crate::*;

/// The Python Options Request is sent from the client to the server to
/// query for the list of the interpreter flags used to run a given list of
/// targets.
#[derive(Debug)]
pub enum BuildTargetPythonOptions {}

impl Request for BuildTargetPythonOptions {
    type Params = PythonOptionsParams;
    type Result = PythonOptionsResult;
    const METHOD: &'static str = "buildTarget/pythonOptions";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_target_python_options_method() {
        assert_eq!(
            BuildTargetPythonOptions::METHOD,
            "buildTarget/pythonOptions"
        );
    }
}
