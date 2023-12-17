use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestResult {
    /// An optional request id to know the origin of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// A status code for the execution.
    pub status_code: StatusCode,
    /// Language-specific metadata about the test result.
    /// See ScalaTestParams as an example.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<TestResultData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    use serde_json::json;

    #[test]
    fn test_result() {
        let test_data = TestResult {
            origin_id: Some(Identifier::default()),
            status_code: StatusCode::default(),
            data: Some(TestResultData::Other(OtherData {
                data: json!({}),
                ..OtherData::default()
            })),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "originId": "",
  "statusCode": 1,
  "dataKind": "",
  "data": {}
}
"#);

        test_deserialization(
            r#"{"originId": "", "statusCode": 1, "dataKind": "", "data": {}}"#,
            &test_data,
        );
    }
}
