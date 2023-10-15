use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseSourcesParams {
    pub text_document: TextDocumentIdentifier,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn inverse_sources_params() {
        assert_json_snapshot!(
           InverseSourcesParams {text_document: TextDocumentIdentifier::default()},
           @r#"
{
  "textDocument": {
    "uri": ""
  }
}
   "#
        );
    }
}
