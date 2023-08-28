use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum DiagnosticSeverity {
    #[default]
    Error = 1,
    Warning = 2,
    Information = 3,
    Hint = 4,
}
