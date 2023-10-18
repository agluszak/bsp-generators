use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CancelRequestParams {
    /// The request id to cancel.
    pub id: RequestId,
}
