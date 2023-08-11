use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathItem {
    /** Either a file or a directory. A directory entry must end with a forward
    slash "/" and a directory entry implies that every nested path within the
    directory belongs to this output item. */
    pub uri: String,
    /** Type of file of the output item, such as whether it is file or directory. */
    pub kind: OutputPathItemKind,
}
