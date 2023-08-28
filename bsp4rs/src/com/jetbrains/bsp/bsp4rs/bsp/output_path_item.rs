use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathItem {
    /** Either a file or a directory. A directory entry must end with a forward
    slash "/" and a directory entry implies that every nested path within the
    directory belongs to this output item. */
    #[serde(default)]
    pub uri: URI,
    /** Type of file of the output item, such as whether it is file or directory. */
    #[serde(default)]
    pub kind: OutputPathItemKind,
}
