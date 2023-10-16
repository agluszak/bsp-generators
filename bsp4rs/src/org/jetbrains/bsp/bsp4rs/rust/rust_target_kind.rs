use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum RustTargetKind {
    #[default]
    /// For lib targets.
    Lib = 1,
    /// For binaries.
    Bin = 2,
    /// For integration tests.
    Test = 3,
    /// For examples.
    Example = 4,
    /// For benchmarks.
    Bench = 5,
    /// For build scripts.
    CustomBuild = 6,
    /// For unknown targets.
    Unknown = 7,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn rust_target_kind() {
        assert_compact_json_snapshot!(
           RustTargetKind::Lib,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &RustTargetKind::Lib);

        assert_compact_json_snapshot!(
           RustTargetKind::Bin,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &RustTargetKind::Bin);

        assert_compact_json_snapshot!(
           RustTargetKind::Test,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &RustTargetKind::Test);

        assert_compact_json_snapshot!(
           RustTargetKind::Example,
           @r#"4"#
        );
        test_deserialization(r#"4"#, &RustTargetKind::Example);

        assert_compact_json_snapshot!(
           RustTargetKind::Bench,
           @r#"5"#
        );
        test_deserialization(r#"5"#, &RustTargetKind::Bench);

        assert_compact_json_snapshot!(
           RustTargetKind::CustomBuild,
           @r#"6"#
        );
        test_deserialization(r#"6"#, &RustTargetKind::CustomBuild);

        assert_compact_json_snapshot!(
           RustTargetKind::Unknown,
           @r#"7"#
        );
        test_deserialization(r#"7"#, &RustTargetKind::Unknown);
    }
}
