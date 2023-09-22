use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesItem {
    /// The build target that contains the test classes.
    pub target: BuildTargetIdentifier,
    /// The main class item.
    pub classes: Vec<ScalaMainClass>,
}
