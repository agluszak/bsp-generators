use serde::{Deserialize, Serialize};

use crate::*;

/// A Scala action represents a change that can be performed in code.
/// See also [LSP: Code Action Request](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction).
///
/// **Note**: In LSP, `CodeAction` appears only as a response to a `textDocument/codeAction` request,
/// whereas ScalaAction is intended to be returned as `Diagnostics.data.actions`.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaAction {
    /// A short, human-readable, title for this code action.
    pub title: String,
    /// A description that may be shown to the user client side to explain the action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// The workspace edit this code action performs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub edit: Option<ScalaWorkspaceEdit>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_action() {
        let test_data = ScalaAction {
            title: TEST_STRING.to_string(),
            description: Some(TEST_STRING.to_string()),
            edit: Some(ScalaWorkspaceEdit::default()),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "title": "test_string",
  "description": "test_string",
  "edit": {
    "changes": []
  }
}
"#);

        test_deserialization(
            r#"{"title": "test_string", "description": "test_string", "edit": {"changes": []}}"#,
            &test_data,
        );
    }
}
