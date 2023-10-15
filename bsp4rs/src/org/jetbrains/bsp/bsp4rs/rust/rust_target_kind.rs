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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn rust_target_kind() {
        assert_json_snapshot!(
           RustTargetKind::Lib,
           @r#"
1
   "#
        );
        assert_json_snapshot!(
           RustTargetKind::Bin,
           @r#"
2
   "#
        );
        assert_json_snapshot!(
           RustTargetKind::Test,
           @r#"
3
   "#
        );
        assert_json_snapshot!(
           RustTargetKind::Example,
           @r#"
4
   "#
        );
        assert_json_snapshot!(
           RustTargetKind::Bench,
           @r#"
5
   "#
        );
        assert_json_snapshot!(
           RustTargetKind::CustomBuild,
           @r#"
6
   "#
        );
        assert_json_snapshot!(
           RustTargetKind::Unknown,
           @r#"
7
   "#
        );
    }
}
