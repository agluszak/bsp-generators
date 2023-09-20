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
