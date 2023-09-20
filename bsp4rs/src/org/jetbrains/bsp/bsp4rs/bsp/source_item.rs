use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourceItem {
    /// Either a text document or a directory. A directory entry must end with a forward
    /// slash "/" and a directory entry implies that every nested text document within the
    /// directory belongs to this source item.
    pub uri: URI,
    /// Type of file of the source item, such as whether it is file or directory.
    pub kind: SourceItemKind,
    /// Indicates if this source is automatically generated by the build and is not
    /// intended to be manually edited by the user.
    pub generated: bool,
}
