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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn source_item() {
        let test_data = SourceItem {
            uri: URI::default(),
            kind: SourceItemKind::default(),
            generated: TEST_BOOL,
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "uri": "",
  "kind": 1,
  "generated": true
}
"#);

        test_deserialization(r#"{"uri": "", "kind": 1, "generated": true}"#, &test_data);
    }
}
