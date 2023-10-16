use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustToolchainItem {
    /// Additional information about Rust toolchain.
    /// Obtained from `rustc`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rust_std_lib: Option<RustcInfo>,
    /// Path to Cargo executable.
    pub cargo_bin_path: URI,
    /// Location of the source code of procedural macros in the Rust toolchain.
    pub proc_macro_srv_path: URI,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn rust_toolchain_item() {
        let test_data = RustToolchainItem {
            rust_std_lib: Some(RustcInfo::default()),
            cargo_bin_path: URI::default(),
            proc_macro_srv_path: URI::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "rustStdLib": {
    "sysrootPath": "",
    "srcSysrootPath": "",
    "version": "",
    "host": ""
  },
  "cargoBinPath": "",
  "procMacroSrvPath": ""
}
"#);

        test_deserialization(
            r#"{"rustStdLib": {"sysrootPath": "", "srcSysrootPath": "", "version": "", "host": ""}, "cargoBinPath": "", "procMacroSrvPath": ""}"#,
            &test_data,
        );
    }
}
