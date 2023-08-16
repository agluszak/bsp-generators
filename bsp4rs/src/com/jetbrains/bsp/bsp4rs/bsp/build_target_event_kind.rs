use serde_repr::{Deserialize_repr, Serialize_repr};

/** The `BuildTargetEventKind` information can be used by clients to trigger
reindexing or update the user interface with the new information. */
#[derive(Debug, PartialEq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BuildTargetEventKind {
    #[default]
    /** The `BuildTargetEventKind` information can be used by clients to trigger
    reindexing or update the user interface with the new information. */
    Created = 1,
    /** The `BuildTargetEventKind` information can be used by clients to trigger
    reindexing or update the user interface with the new information. */
    Changed = 2,
    /** The `BuildTargetEventKind` information can be used by clients to trigger
    reindexing or update the user interface with the new information. */
    Deleted = 3,
}
