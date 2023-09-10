use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesItem {
    /** The build target that contains the test classes. */
    #[serde(default)]
    pub target: BuildTargetIdentifier,
    /** Name of the the framework to which classes belong.
    It's optional in order to maintain compatibility, however it is expected
    from the newer implementations to not leave that field unspecified. */
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub framework: Option<String>,
    /** The fully qualified names of the test classes in this target */
    #[serde(default)]
    pub classes: Vec<String>,
}
