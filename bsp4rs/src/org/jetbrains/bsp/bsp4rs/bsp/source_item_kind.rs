use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum SourceItemKind {
    /// The source item references a normal file.
    #[default]
    File = 1,
    /// The source item references a directory.
    Directory = 2,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn source_item_kind() {
        assert_compact_json_snapshot!(
           SourceItemKind::File,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &SourceItemKind::File);

        assert_compact_json_snapshot!(
           SourceItemKind::Directory,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &SourceItemKind::Directory);
    }
}
