use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileResult {
    /// An optional request id to know the origin of this report.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// A status code for the execution.
    pub status_code: StatusCode,
    /// A field containing language-specific information, like products
    /// of compilation or compiler-specific metadata the client needs to know.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<CompileResultData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn compile_result() {
        let test_data = CompileResult {
            origin_id: Some(Identifier::default()),
            status_code: StatusCode::default(),
            data: Some(CompileResultData::Other(OtherData::default())),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "originId": "",
  "statusCode": 1,
  "dataKind": "",
  "data": null
}
"#);

        test_deserialization(
            r#"{"originId": "", "statusCode": 1, "dataKind": "", "data": null}"#,
            &test_data,
        );
    }
}
