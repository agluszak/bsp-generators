use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    /** An error message. */
    Error = 1,
    /** A warning message. */
    Warning = 2,
    /** An information message. */
    Info = 3,
    /** A log message. */
    Log = 4,
}
