use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestSuites {
    /** The fully qualified names of the test classes in this target and the tests in this test classes */
    pub suites: Vec<ScalaTestSuiteSelection>,
    /** Additional jvmOptions which will be passed to the forked JVM */
    pub jvm_options: Vec<String>,
    /** Enviroment variables should be an array of strings in format KEY=VALUE */
    pub environment_variables: Vec<String>,
}
