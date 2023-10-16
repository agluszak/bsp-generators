use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum TestStatus {
    #[default]
    /// The test passed successfully.
    Passed = 1,
    /// The test failed.
    Failed = 2,
    /// The test was marked as ignored.
    Ignored = 3,
    /// The test execution was cancelled.
    Cancelled = 4,
    /// The was not included in execution.
    Skipped = 5,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn test_status() {
        assert_compact_json_snapshot!(
           TestStatus::Passed,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &TestStatus::Passed);

        assert_compact_json_snapshot!(
           TestStatus::Failed,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &TestStatus::Failed);

        assert_compact_json_snapshot!(
           TestStatus::Ignored,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &TestStatus::Ignored);

        assert_compact_json_snapshot!(
           TestStatus::Cancelled,
           @r#"4"#
        );
        test_deserialization(r#"4"#, &TestStatus::Cancelled);

        assert_compact_json_snapshot!(
           TestStatus::Skipped,
           @r#"5"#
        );
        test_deserialization(r#"5"#, &TestStatus::Skipped);
    }
}
