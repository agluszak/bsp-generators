use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathItem {
    /// Either a file or a directory. A directory entry must end with a forward
    /// slash "/" and a directory entry implies that every nested path within the
    /// directory belongs to this output item.
    pub uri: URI,
    /// Type of file of the output item, such as whether it is file or directory.
    pub kind: OutputPathItemKind,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn output_path_item() {
        assert_json_snapshot!(
           OutputPathItem {uri: URI::default(), kind: OutputPathItemKind::default()},
           @r#"
{
  "uri": "",
  "kind": 1
}
   "#
        );
    }
}
