use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseSourcesParams {
    pub text_document: TextDocumentIdentifier,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn inverse_sources_params() {
        let test_data = InverseSourcesParams {
            text_document: TextDocumentIdentifier::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "textDocument": {
    "uri": ""
  }
}
"#);

        test_deserialization(r#"{"textDocument": {"uri": ""}}"#, &test_data);
    }
}
