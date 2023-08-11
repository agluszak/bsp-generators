use crate::*;

#[derive(Debug)]
pub enum BuildTargetJavacOptions {}

impl Request for BuildTargetJavacOptions {
    type Params = JavacOptionsParams;
    type Result = JavacOptionsResult;
    const METHOD: &'static str = "buildTarget/javacOptions";
}
