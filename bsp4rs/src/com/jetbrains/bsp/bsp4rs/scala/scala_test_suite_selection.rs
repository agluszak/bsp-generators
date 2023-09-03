use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestSuiteSelection {
    /** Fully qualified name of the test suite class */
    #[serde(default)]
    pub class_name: String,
    /** List of tests which should be run within this test suite.
    Empty collection means that all of them are supposed to be executed. */
    #[serde(default)]
    pub tests: Vec<String>,
}
