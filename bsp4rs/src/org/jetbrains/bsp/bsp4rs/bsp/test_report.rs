use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestReport {
    #[deprecated(note = "Use the field in TaskFinishParams instead")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// The build target that was compiled.
    pub target: BuildTargetIdentifier,
    /// The total number of successful tests.
    pub passed: i32,
    /// The total number of failed tests.
    pub failed: i32,
    /// The total number of ignored tests.
    pub ignored: i32,
    /// The total number of cancelled tests.
    pub cancelled: i32,
    /// The total number of skipped tests.
    pub skipped: i32,
    /// The total number of milliseconds tests take to run (e.g. doesn't include compile times).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn test_report() {
        let test_data = TestReport {
            origin_id: Some(Identifier::default()),
            target: BuildTargetIdentifier::default(),
            passed: TEST_INT,
            failed: TEST_INT,
            ignored: TEST_INT,
            cancelled: TEST_INT,
            skipped: TEST_INT,
            time: Some(TEST_LONG),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "originId": "",
  "target": {
    "uri": ""
  },
  "passed": 1,
  "failed": 1,
  "ignored": 1,
  "cancelled": 1,
  "skipped": 1,
  "time": 2
}
   "#
        );

        test_deserialization(
            r#"{"originId": "", "target": {"uri": ""}, "passed": 1, "failed": 1, "ignored": 1, "cancelled": 1, "skipped": 1, "time": 2}"#,
            &test_data,
        );
    }
}
