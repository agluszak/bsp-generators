use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestSuiteSelection {
    /** Fully qualified name of the test suite class */
    pub class_name: String,
    /** List of tests which should be run within this test suite.
    Empty collection means that all of them are supposed to be executed. */
    pub tests: Vec<String>,
}
