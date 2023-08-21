use serde::{Deserialize, Serialize};

use crate::*;

/** A Scala action represents a change that can be performed in code.
See also [LSP: Code Action Request](https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocument_codeAction).

**Note**: In LSP, `CodeAction` appears only as a response to a `textDocument/codeAction` request,
whereas ScalaAction is intended to be returned as `Diagnostics.data.actions`. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaAction {
    /** A short, human-readable, title for this code action. */
    #[serde(default)]
    pub title: String,
    /** A description that may be shown to the user client side to explain the action. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /** The workspace edit this code action performs. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub edit: Option<ScalaWorkspaceEdit>,
}
