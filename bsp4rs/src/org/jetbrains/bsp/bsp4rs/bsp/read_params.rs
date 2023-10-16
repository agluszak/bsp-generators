use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ReadParams {
    /// The id of the request.
    pub origin_id: Identifier,
    /// Relevant only for test tasks.
    /// Allows to tell the client from which task the output is coming from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskId>,
    /// Message content can contain arbitrary bytes.
    /// They should be escaped as per [javascript encoding](https://developer.mozilla.org/en-US/docs/Web/JavaScript/Guide/Grammar_and_types#using_special_characters_in_strings)
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn read_params() {
        let test_data = ReadParams {
            origin_id: Identifier::default(),
            task: Some(TaskId::default()),
            message: TEST_STRING.to_string(),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "originId": "",
  "task": {
    "id": ""
  },
  "message": "test_string"
}
   "#
        );

        test_deserialization(
            r#"{"originId": "", "task": {"id": ""}, "message": "test_string"}"#,
            &test_data,
        );
    }
}
