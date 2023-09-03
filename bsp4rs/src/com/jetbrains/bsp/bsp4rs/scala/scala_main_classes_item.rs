use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesItem {
    /** The build target that contains the test classes. */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** The main class item. */
    #[serde(default)]
    pub classes: Vec<ScalaMainClass>,
}
