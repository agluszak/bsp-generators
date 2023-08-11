use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskProgressParams {
    /** Unique id of the task with optional reference to parent task id */
    pub task_id: TaskId,
    /** Timestamp of when the event started in milliseconds since Epoch. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_time: Option<i64>,
    /** Message describing the task. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /** If known, total amount of work units in this task. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<i64>,
    /** If known, completed amount of work units in this task. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub progress: Option<i64>,
    /** Name of a work unit. For example, "files" or "tests". May be empty. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit: Option<String>,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** Optional metadata about the task.
    Objects for specific tasks like compile, test, etc are specified in the protocol. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}