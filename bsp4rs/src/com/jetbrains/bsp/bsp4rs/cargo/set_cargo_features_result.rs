use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCargoFeaturesResult {
    /** The status code of the operation. */
    #[serde(default)]
    pub status_code: StatusCode,
}
