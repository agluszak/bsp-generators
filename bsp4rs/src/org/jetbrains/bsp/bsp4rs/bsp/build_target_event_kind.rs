use serde_repr::{Deserialize_repr, Serialize_repr};

/// The `BuildTargetEventKind` information can be used by clients to trigger
/// reindexing or update the user interface with the new information.
#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum BuildTargetEventKind {
    /// The build target is new.
    #[default]
    Created = 1,
    /// The build target has changed.
    Changed = 2,
    /// The build target has been deleted.
    Deleted = 3,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn build_target_event_kind() {
        assert_compact_json_snapshot!(
           BuildTargetEventKind::Created,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &BuildTargetEventKind::Created);

        assert_compact_json_snapshot!(
           BuildTargetEventKind::Changed,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &BuildTargetEventKind::Changed);

        assert_compact_json_snapshot!(
           BuildTargetEventKind::Deleted,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &BuildTargetEventKind::Deleted);
    }
}
