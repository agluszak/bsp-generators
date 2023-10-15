use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustcInfo {
    /// Root directory where the Rust compiler looks for standard libraries and other
    /// essential components when building Rust projects.
    pub sysroot_path: URI,
    /// Source code for the Rust standard library.
    pub src_sysroot_path: URI,
    /// `rustc` SemVer (Semantic Versioning) version.
    pub version: String,
    /// Target architecture and operating system of the Rust compiler.
    /// Used by [`intellij-rust`] for checking if given toolchain is supported.
    pub host: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn rustc_info() {
        assert_json_snapshot!(
           RustcInfo {sysroot_path: URI::default(), src_sysroot_path: URI::default(), version: TEST_STRING.to_string(), host: TEST_STRING.to_string()},
           @r#"
{
  "sysrootPath": "",
  "srcSysrootPath": "",
  "version": "test_string",
  "host": "test_string"
}
   "#
        );
    }
}
