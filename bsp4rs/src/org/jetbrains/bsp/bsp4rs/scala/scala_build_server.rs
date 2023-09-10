use crate::*;

#[derive(Debug)]
pub enum BuildTargetScalacOptions {}

impl Request for BuildTargetScalacOptions {
    type Params = ScalacOptionsParams;
    type Result = ScalacOptionsResult;
    const METHOD: &'static str = "buildTarget/scalacOptions";
}

#[derive(Debug)]
pub enum BuildTargetScalaTestClasses {}

impl Request for BuildTargetScalaTestClasses {
    type Params = ScalaTestClassesParams;
    type Result = ScalaTestClassesResult;
    const METHOD: &'static str = "buildTarget/scalaTestClasses";
}

#[derive(Debug)]
pub enum BuildTargetScalaMainClasses {}

impl Request for BuildTargetScalaMainClasses {
    type Params = ScalaMainClassesParams;
    type Result = ScalaMainClassesResult;
    const METHOD: &'static str = "buildTarget/scalaMainClasses";
}
