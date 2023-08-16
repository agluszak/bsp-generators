use serde::{Deserialize, Serialize};

/** Task progress notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field. */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskProgressDataKind {}
