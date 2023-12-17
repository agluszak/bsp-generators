use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestFinish {
    /// Name or description of the test.
    pub display_name: String,
    /// Information about completion of the test, for example an error message.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Completion status of the test.
    pub status: TestStatus,
    /// Source location of the test, as LSP location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
    /// Optionally, structured metadata about the test completion.
    /// For example: stack traces, expected/actual values.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<TestFinishData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    use serde_json::json;

    #[test]
    fn test_finish() {
        let test_data = TestFinish {
            display_name: TEST_STRING.to_string(),
            message: Some(TEST_STRING.to_string()),
            status: TestStatus::default(),
            location: Some(Location::default()),
            data: Some(TestFinishData::Other(OtherData {
                data: json!({}),
                ..OtherData::default()
            })),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "displayName": "test_string",
  "message": "test_string",
  "status": 1,
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
  },
  "dataKind": "",
  "data": {}
}
"#);

        test_deserialization(
            r#"{"displayName": "test_string", "message": "test_string", "status": 1, "location": {"uri": "", "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}}, "dataKind": "", "data": {}}"#,
            &test_data,
        );
    }
}
