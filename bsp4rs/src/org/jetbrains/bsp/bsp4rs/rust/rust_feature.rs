use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustFeature {
    /** Name of the feature. */
    pub name: Feature,
    /** Feature's dependencies. */
    pub dependencies: Vec<Feature>,
}
