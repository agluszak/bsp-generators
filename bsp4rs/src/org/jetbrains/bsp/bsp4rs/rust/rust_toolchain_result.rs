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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn rust_toolchain_result() {
        assert_json_snapshot!(
           RustToolchainResult {toolchains: vec![RustToolchainItem::default()]},
           @r#"
{
  "toolchains": [
    {
      "cargoBinPath": "",
      "procMacroSrvPath": ""
    }
  ]
}
   "#
        );
    }
}
