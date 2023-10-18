use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRequestParams {
    /// The request id to cancel.
    pub id: RequestId,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn cancel_request_params() {
        let test_data = CancelRequestParams {
            id: RequestId::String(String::default()),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "id": ""
}
"#);

        test_deserialization(r#"{"id": ""}"#, &test_data);
    }
}
