use serde::{Deserialize, Serialize};

use crate::*;

/** The Task Id allows clients to _uniquely_ identify a BSP task and establish a client-parent relationship with another task id. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TaskId {
    /** A unique identifier */
    #[serde(default)]
    pub id: Identifier,
    /** The parent task ids, if any. A non-empty parents field means
    this task is a sub-task of every parent task id. The child-parent
    relationship of tasks makes it possible to render tasks in
    a tree-like user interface or inspect what caused a certain task
    execution. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub parents: Vec<Identifier>,
}
