use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum TestStatus {
    #[default]
    Passed = 1,
    Failed = 2,
    Ignored = 3,
    Cancelled = 4,
    Skipped = 5,
}
