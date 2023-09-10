use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::{BTreeMap, BTreeSet};

/** The feature dependency graph is a mapping between
feature and the features it turns on */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeatureDependencyGraph(pub BTreeMap<Feature, BTreeSet<Feature>>);

impl std::ops::Deref for FeatureDependencyGraph {
    type Target = BTreeMap<Feature, BTreeSet<Feature>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BTreeMap<Feature, BTreeSet<Feature>>> for FeatureDependencyGraph {
    fn from(input: BTreeMap<Feature, BTreeSet<Feature>>) -> Self {
        FeatureDependencyGraph(input)
    }
}
