use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum MessageType {
    #[default]
    Error = 1,
    Warning = 2,
    Info = 3,
    Log = 4,
}
