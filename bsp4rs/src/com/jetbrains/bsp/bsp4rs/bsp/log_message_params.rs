use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LogMessageParams {
    /** the message type. */
    #[serde(default)]
    pub r#type: MessageType,
    /** The task id if any. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub task: Option<TaskId>,
    /** The request id that originated this notification.
    The originId field helps clients know which request originated a notification in case several requests are handled by the
    client at the same time. It will only be populated if the client defined it in the request that triggered this notification. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<RequestId>,
    /** The actual message. */
    #[serde(default)]
    pub message: String,
}
