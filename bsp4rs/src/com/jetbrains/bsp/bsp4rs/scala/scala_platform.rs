use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum ScalaPlatform {
    Jvm = 1,
    Js = 2,
    Native = 3,
}
