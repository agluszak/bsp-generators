use std::collections::BTreeMap;

/** The feature dependency graph is a mapping between
feature and the features it turns on */
pub type FeatureDependencyGraph = BTreeMap<String, Vec<String>>;
