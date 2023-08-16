use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum SourceItemKind {
    #[default]
    File = 1,
    Directory = 2,
}
