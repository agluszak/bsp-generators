use serde::{Deserialize, Serialize};

use crate::*;

/// `ScalaDiagnostic` is a data structure that contains Scala-specific
/// metadata generated by Scala compilation.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaDiagnostic {
    /// Actions (also known as quick fixes) that are able to either fix or address
    /// the issue that is causing this diagnostic.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub actions: Option<Vec<ScalaAction>>,
}

#[cfg(test)]
mod tests {}
