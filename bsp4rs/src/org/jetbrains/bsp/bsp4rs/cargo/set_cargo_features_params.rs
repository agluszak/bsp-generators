use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCargoFeaturesParams {
    /// Package ID for which new features state will be set.
    pub package_id: String,
    /// The list of features to be set as a new state.
    pub features: BTreeSet<Feature>,
}

#[cfg(test)]
mod tests {}
