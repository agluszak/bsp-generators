use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /** The range at which the message applies. */
    pub range: Range,
    /** The diagnostic's severity. Can be omitted. If omitted it is up to the
    client to interpret diagnostics as error, warning, info or hint. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosticSeverity>,
    /** The diagnostic's code, which might appear in the user interface. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /** A human-readable string describing the source of this
    diagnostic, e.g. 'typescript' or 'super lint'. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /** The diagnostic's message. */
    pub message: String,
    /** An array of related diagnostic information, e.g. when symbol-names within
    a scope collide all definitions can be marked via this property. */
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub related_information: Vec<DiagnosticRelatedInformation>,
    /** Kind of data to expect in the `data` field. If this field is not set, the kind of data is not specified. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_kind: Option<String>,
    /** A data entry field that is preserved between a `textDocument/publishDiagnostics` notification
    and a `textDocument/codeAction` request. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}
