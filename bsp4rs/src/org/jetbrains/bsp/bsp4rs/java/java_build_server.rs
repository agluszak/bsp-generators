use crate::*;

#[derive(Debug)]
pub enum BuildTargetJavacOptions {}

/** The build target javac options request is sent from the client to the server to
query for the list of compiler options necessary to compile in a given list of
targets. */
impl Request for BuildTargetJavacOptions {
    type Params = JavacOptionsParams;
    type Result = JavacOptionsResult;
    const METHOD: &'static str = "buildTarget/javacOptions";
}
