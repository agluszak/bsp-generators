use serde::{Deserialize, Serialize};

/** The debug session will connect to a running process. The DAP client will send the port of the running process later. */
#[derive(Debug, PartialEq, Eq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaAttachRemote {}
