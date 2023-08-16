use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DiagnosticTag {
    #[default]
    Unnecessary = 1,
    Deprecated = 2,
}
