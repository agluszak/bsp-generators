use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LibraryItem {
    pub id: BuildTargetIdentifier,
    pub dependencies: Vec<BuildTargetIdentifier>,
    pub jars: Vec<Jar>,
}
