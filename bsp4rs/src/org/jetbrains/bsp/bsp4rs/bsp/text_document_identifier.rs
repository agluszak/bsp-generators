use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TextDocumentIdentifier {
    /// The text document's URI.
    pub uri: URI,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn text_document_identifier() {
        assert_json_snapshot!(
           TextDocumentIdentifier {uri: URI::default()},
           @r#"
{
  "uri": ""
}
   "#
        );
    }
}
