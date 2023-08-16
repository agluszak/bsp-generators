use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestStart {
    /** Name or description of the test. */
    pub display_name: String,
    /** Source location of the test, as LSP location. */
    #[serde(skip_serializing_if = "Option::is_none")]
    pub location: Option<Location>,
}
