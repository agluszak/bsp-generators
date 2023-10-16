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
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn debug_session_address() {
        let test_data = DebugSessionAddress {
            uri: URI::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "uri": ""
}
"#);

        test_deserialization(r#"{"uri": ""}"#, &test_data);
    }
}
