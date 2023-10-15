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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn run_result() {
        assert_json_snapshot!(
           RunResult {origin_id: Some(Identifier::default()), status_code: StatusCode::default()},
           @r#"
{
  "originId": "",
  "statusCode": 1
}
   "#
        );
    }
}
