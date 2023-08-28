use serde_repr::{Deserialize_repr, Serialize_repr};

/** The `BuildTargetEventKind` information can be used by clients to trigger
reindexing or update the user interface with the new information. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize_repr, Deserialize_repr)]
#[repr(u8)]
pub enum BuildTargetEventKind {
    #[default]
    /** The build target is new. */
    Created = 1,
    /** The build target has changed. */
    Changed = 2,
    /** The build target has been deleted. */
    Deleted = 3,
}
