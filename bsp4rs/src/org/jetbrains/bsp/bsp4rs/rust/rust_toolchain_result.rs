use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustToolchainResult {
    /// A sequence of Rust toolchains.
    pub toolchains: Vec<RustToolchainItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn rust_toolchain_result() {
        let test_data = RustToolchainResult {
            toolchains: vec![RustToolchainItem::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "toolchains": [
    {
      "cargoBinPath": "",
      "procMacroSrvPath": ""
    }
  ]
}
"#);

        test_deserialization(
            r#"{"toolchains": [{"cargoBinPath": "", "procMacroSrvPath": ""}]}"#,
            &test_data,
        );
    }
}
