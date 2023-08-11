use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum MessageType {
    Error = 1,
    Warning = 2,
    Info = 3,
    Log = 4,
}
