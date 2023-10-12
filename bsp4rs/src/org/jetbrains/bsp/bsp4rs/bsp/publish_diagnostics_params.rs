use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PublishDiagnosticsParams {
    /// The document where the diagnostics are published.
    pub text_document: TextDocumentIdentifier,
    /// The build target where the diagnostics origin.
    /// It is valid for one text document to belong to multiple
    /// build targets, for example sources that are compiled against multiple
    /// platforms (JVM, JavaScript).
    pub build_target: BuildTargetIdentifier,
    /// The request id that originated this notification.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<RequestId>,
    /// The diagnostics to be published by the client.
    pub diagnostics: Vec<Diagnostic>,
    /// Whether the client should clear the previous diagnostics
    /// mapped to the same `textDocument` and `buildTarget`.
    pub reset: bool,
}

#[cfg(test)]
mod tests {}
