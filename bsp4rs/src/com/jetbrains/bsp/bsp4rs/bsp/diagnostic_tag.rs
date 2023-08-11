use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Serialize_repr, Deserialize_repr, Clone)]
#[repr(u8)]
pub enum DiagnosticTag {
    Unnecessary = 1,
    Deprecated = 2,
}
