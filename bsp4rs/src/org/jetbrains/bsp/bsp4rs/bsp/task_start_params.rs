use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStartParams {
    /// Unique id of the task with optional reference to parent task id
    pub task_id: TaskId,
    /// A unique identifier generated by the client to identify this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// Timestamp of when the event started in milliseconds since Epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<i64>,
    /// Message describing the task.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Optional metadata about the task.
    /// Objects for specific tasks like compile, test, etc are specified in the protocol.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskStartData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn task_start_params() {
        let test_data = TaskStartParams {
            task_id: TaskId::default(),
            origin_id: Some(Identifier::default()),
            event_time: Some(TEST_LONG),
            message: Some(TEST_STRING.to_string()),
            data: Some(TaskStartData::Other(OtherData::default())),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "taskId": {
    "id": ""
  },
  "originId": "",
  "eventTime": 2,
  "message": "test_string",
  "dataKind": "",
  "data": null
}
"#);

        test_deserialization(
            r#"{"taskId": {"id": ""}, "originId": "", "eventTime": 2, "message": "test_string", "dataKind": "", "data": null}"#,
            &test_data,
        );
    }
}
