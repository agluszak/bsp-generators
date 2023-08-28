use crate::*;
use std::collections::{BTreeMap, BTreeSet};

/** The feature dependency graph is a mapping between
feature and the features it turns on */
pub type FeatureDependencyGraph = BTreeMap<Feature, BTreeSet<Feature>>;
