use serde::{Deserialize, Serialize};

use crate::*;

/** Represents a related message and source code location for a diagnostic.
This should be used to point to code locations that cause or are related to
a diagnostics, e.g when duplicating a symbol in a scope. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRelatedInformation {
    /** The location of this related diagnostic information. */
    #[serde(default)]
    pub location: Location,
    /** The message of this related diagnostic information. */
    #[serde(default)]
    pub message: String,
}
