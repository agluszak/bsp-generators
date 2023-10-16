use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::{BTreeMap, BTreeSet};

/// The feature dependency graph is a mapping between
/// feature and the features it turns on
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeaturesDependencyGraph(pub BTreeMap<Feature, BTreeSet<Feature>>);

impl std::ops::Deref for FeaturesDependencyGraph {
    type Target = BTreeMap<Feature, BTreeSet<Feature>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BTreeMap<Feature, BTreeSet<Feature>>> for FeaturesDependencyGraph {
    fn from(input: BTreeMap<Feature, BTreeSet<Feature>>) -> Self {
        Self(input)
    }
}