use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskStartParams {
    /** Unique id of the task with optional reference to parent task id */
    #[serde(default)]
    pub task_id: TaskId,
    /** Timestamp of when the event started in milliseconds since Epoch. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_time: Option<i64>,
    /** Message describing the task. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /** Optional metadata about the task.
    Objects for specific tasks like compile, test, etc are specified in the protocol. */
    #[serde(default, flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskStartData>,
}
