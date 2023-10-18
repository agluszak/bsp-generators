use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunResult {
    /// An optional request id to know the origin of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// A status code for the execution.
    pub status_code: StatusCode,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn run_result() {
        let test_data = RunResult {
            origin_id: Some(Identifier::default()),
            status_code: StatusCode::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "originId": "",
  "statusCode": 1
}
"#);

        test_deserialization(r#"{"originId": "", "statusCode": 1}"#, &test_data);
    }
}
