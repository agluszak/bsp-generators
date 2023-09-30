use serde::{Deserialize, Serialize};

use crate::*;

/// The Task Id allows clients to _uniquely_ identify a BSP task and establish a client-parent relationship with another task id.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskId {
    /// A unique identifier
    pub id: Identifier,
    /// The parent task ids, if any. A non-empty parents field means
    /// this task is a sub-task of every parent task id. The child-parent
    /// relationship of tasks makes it possible to render tasks in
    /// a tree-like user interface or inspect what caused a certain task
    /// execution.
    /// OriginId should not be included in the parents field, there is a separate
    /// field for that.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parents: Option<Vec<Identifier>>,
}
