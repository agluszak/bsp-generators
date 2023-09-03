use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport {
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /** The build target that was compiled. */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** The total number of successful tests. */
    #[serde(default)]
    pub passed: i32,
    /** The total number of failed tests. */
    #[serde(default)]
    pub failed: i32,
    /** The total number of ignored tests. */
    #[serde(default)]
    pub ignored: i32,
    /** The total number of cancelled tests. */
    #[serde(default)]
    pub cancelled: i32,
    /** The total number of skipped tests. */
    #[serde(default)]
    pub skipped: i32,
    /** The total number of milliseconds tests take to run (e.g. doesn't include compile times). */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
