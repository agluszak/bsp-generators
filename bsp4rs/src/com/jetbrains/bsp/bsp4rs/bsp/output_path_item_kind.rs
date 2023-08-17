use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum OutputPathItemKind {
    #[default]
    /** The output path item references a normal file. */
    File = 1,
    /** The output path item references a directory. */
    Directory = 2,
}
