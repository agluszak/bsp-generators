use serde::{Deserialize, Serialize};

use crate::*;

/** `ScalaTestParams` contains scala-specific metadata for testing Scala targets. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestParams {
    /** The test classes to be run in this test execution.
    It is the result of `buildTarget/scalaTestClasses`. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub test_classes: Vec<ScalaTestClassesItem>,
    /** The JVM options to run tests with. They replace any options
    that are defined by the build server if defined. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub jvm_options: Vec<String>,
}
