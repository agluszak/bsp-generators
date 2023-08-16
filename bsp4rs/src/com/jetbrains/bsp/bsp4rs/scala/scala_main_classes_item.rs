use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesItem {
    /** The build target that contains the test classes. */
    pub target: BuildTargetIdentifier,
    /** The main class item. */
    pub classes: Vec<ScalaMainClass>,
}
