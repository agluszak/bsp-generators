use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum OutputPathItemKind {
    #[default]
    /// The output path item references a normal file.
    File = 1,
    /// The output path item references a directory.
    Directory = 2,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn output_path_item_kind() {
        assert_compact_json_snapshot!(
           OutputPathItemKind::File,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &OutputPathItemKind::File);

        assert_compact_json_snapshot!(
           OutputPathItemKind::Directory,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &OutputPathItemKind::Directory);
    }
}
