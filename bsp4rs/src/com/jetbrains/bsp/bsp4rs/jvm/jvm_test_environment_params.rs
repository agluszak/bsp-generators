use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JvmTestEnvironmentParams {
    pub targets: Vec<BuildTargetIdentifier>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}
