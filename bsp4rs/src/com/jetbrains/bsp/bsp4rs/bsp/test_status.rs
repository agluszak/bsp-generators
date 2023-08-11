use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum TestStatus {
    Passed = 1,
    Failed = 2,
    Ignored = 3,
    Cancelled = 4,
    Skipped = 5,
}
