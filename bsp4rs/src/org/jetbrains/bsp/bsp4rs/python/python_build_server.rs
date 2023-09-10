use crate::*;

#[derive(Debug)]
pub enum BuildTargetPythonOptions {}

impl Request for BuildTargetPythonOptions {
    type Params = PythonOptionsParams;
    type Result = PythonOptionsResult;
    const METHOD: &'static str = "buildTarget/pythonOptions";
}
