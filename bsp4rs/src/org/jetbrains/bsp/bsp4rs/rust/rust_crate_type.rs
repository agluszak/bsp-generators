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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn rust_crate_type() {
        assert_json_snapshot!(
           RustCrateType::Bin,
           @r#"
1
   "#
        );
        assert_json_snapshot!(
           RustCrateType::Lib,
           @r#"
2
   "#
        );
        assert_json_snapshot!(
           RustCrateType::Rlib,
           @r#"
3
   "#
        );
        assert_json_snapshot!(
           RustCrateType::Dylib,
           @r#"
4
   "#
        );
        assert_json_snapshot!(
           RustCrateType::Cdylib,
           @r#"
5
   "#
        );
        assert_json_snapshot!(
           RustCrateType::Staticlib,
           @r#"
6
   "#
        );
        assert_json_snapshot!(
           RustCrateType::ProcMacro,
           @r#"
7
   "#
        );
        assert_json_snapshot!(
           RustCrateType::Unknown,
           @r#"
8
   "#
        );
    }
}
