use serde_repr::{Deserialize_repr, Serialize_repr};

/** Included in notifications of tasks or requests to signal the completion state. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StatusCode {
    #[default]
    /** Execution was successful. */
    Ok = 1,
    /** Execution failed. */
    Error = 2,
    /** Execution was cancelled. */
    Cancelled = 3,
}
