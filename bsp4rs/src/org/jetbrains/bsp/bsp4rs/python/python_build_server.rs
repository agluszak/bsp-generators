use crate::*;

#[derive(Debug)]
pub enum BuildTargetPythonOptions {}

/** The Python Options Request is sent from the client to the server to
query for the list of the interpreter flags used to run a given list of
targets. */
impl Request for BuildTargetPythonOptions {
    type Params = PythonOptionsParams;
    type Result = PythonOptionsResult;
    const METHOD: &'static str = "buildTarget/pythonOptions";
}
