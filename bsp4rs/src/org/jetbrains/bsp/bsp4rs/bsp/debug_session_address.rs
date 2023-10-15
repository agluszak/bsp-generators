use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugSessionAddress {
    /// The Debug Adapter Protocol server's connection uri
    pub uri: URI,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn debug_session_address() {
        assert_json_snapshot!(
           DebugSessionAddress {uri: URI::default()},
           @r#"
{
  "uri": ""
}
   "#
        );
    }
}
