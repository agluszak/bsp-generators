use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
    /** The build target that was compiled. */
    pub target: BuildTargetIdentifier,
    /** The total number of successful tests. */
    pub passed: i32,
    /** The total number of failed tests. */
    pub failed: i32,
    /** The total number of ignored tests. */
    pub ignored: i32,
    /** The total number of cancelled tests. */
    pub cancelled: i32,
    /** The total number of skipped tests. */
    pub skipped: i32,
    /** The total number of milliseconds tests take to run (e.g. doesn't include compile times). */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}
