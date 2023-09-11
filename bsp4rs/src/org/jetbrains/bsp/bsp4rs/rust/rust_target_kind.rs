use serde_repr::{Deserialize_repr, Serialize_repr};

#[derive(
    Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize_repr, Deserialize_repr,
)]
#[repr(u8)]
pub enum RustTargetKind {
    #[default]
    /** For lib targets. */
    Lib = 1,
    /** For binaries. */
    Bin = 2,
    /** For integration tests. */
    Test = 3,
    /** For examples. */
    Example = 4,
    /** For benchmarks. */
    Bench = 5,
    /** For build scripts. */
    Custombuild = 6,
    /** For unknown targets. */
    Unknown = 7,
}
