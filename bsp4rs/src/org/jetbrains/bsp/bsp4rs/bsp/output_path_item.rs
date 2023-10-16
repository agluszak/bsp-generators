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
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn output_path_item() {
        let test_data = OutputPathItem {
            uri: URI::default(),
            kind: OutputPathItemKind::default(),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "uri": "",
  "kind": 1
}
   "#
        );

        test_deserialization(r#"{"uri": "", "kind": 1}"#, &test_data);
    }
}
