use serde::{Deserialize, Serialize};

/** `PythonBuildTarget` is a basic data structure that contains Python-specific
metadata, specifically the interpreter reference and the Python version. */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonBuildTarget {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interpreter: Option<String>,
}
