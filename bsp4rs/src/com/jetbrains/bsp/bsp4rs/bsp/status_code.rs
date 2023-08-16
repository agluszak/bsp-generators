use serde_repr::{Deserialize_repr, Serialize_repr};

/** Included in notifications of tasks or requests to signal the completion state. */
#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum StatusCode {
    #[default]
    /** Included in notifications of tasks or requests to signal the completion state. */
    Ok = 1,
    /** Included in notifications of tasks or requests to signal the completion state. */
    Error = 2,
    /** Included in notifications of tasks or requests to signal the completion state. */
    Cancelled = 3,
}
