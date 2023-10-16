use serde_repr::{Deserialize_repr, Serialize_repr};

/// Crate types (`lib`, `rlib`, `dylib`, `cdylib`, `staticlib`) are listed for
/// `lib` and `example` target kinds. For other target kinds `bin` crate type is listed.
#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum RustCrateType {
    #[default]
    Bin = 1,
    Lib = 2,
    Rlib = 3,
    Dylib = 4,
    Cdylib = 5,
    Staticlib = 6,
    ProcMacro = 7,
    Unknown = 8,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_compact_json_snapshot;

    #[test]
    fn rust_crate_type() {
        assert_compact_json_snapshot!(
           RustCrateType::Bin,
           @r#"1"#
        );
        test_deserialization(r#"1"#, &RustCrateType::Bin);

        assert_compact_json_snapshot!(
           RustCrateType::Lib,
           @r#"2"#
        );
        test_deserialization(r#"2"#, &RustCrateType::Lib);

        assert_compact_json_snapshot!(
           RustCrateType::Rlib,
           @r#"3"#
        );
        test_deserialization(r#"3"#, &RustCrateType::Rlib);

        assert_compact_json_snapshot!(
           RustCrateType::Dylib,
           @r#"4"#
        );
        test_deserialization(r#"4"#, &RustCrateType::Dylib);

        assert_compact_json_snapshot!(
           RustCrateType::Cdylib,
           @r#"5"#
        );
        test_deserialization(r#"5"#, &RustCrateType::Cdylib);

        assert_compact_json_snapshot!(
           RustCrateType::Staticlib,
           @r#"6"#
        );
        test_deserialization(r#"6"#, &RustCrateType::Staticlib);

        assert_compact_json_snapshot!(
           RustCrateType::ProcMacro,
           @r#"7"#
        );
        test_deserialization(r#"7"#, &RustCrateType::ProcMacro);

        assert_compact_json_snapshot!(
           RustCrateType::Unknown,
           @r#"8"#
        );
        test_deserialization(r#"8"#, &RustCrateType::Unknown);
    }
}
