use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStart {
    /// Name or description of the test.
    pub display_name: String,
    /// Source location of the test, as LSP location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn test_start() {
        let test_data = TestStart {
            display_name: TEST_STRING.to_string(),
            location: Some(Location::default()),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "displayName": "test_string",
  "location": {
    "uri": "",
    "range": {
      "start": {
        "line": 0,
        "character": 0
      },
      "end": {
        "line": 0,
        "character": 0
      }
    }
  }
}
"#);

        test_deserialization(
            r#"{"displayName": "test_string", "location": {"uri": "", "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}}}"#,
            &test_data,
        );
    }
}
