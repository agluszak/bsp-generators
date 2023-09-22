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
