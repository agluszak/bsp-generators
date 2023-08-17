use std::collections::HashMap;

/** The feature dependency graph is a mapping between
feature and the features it turns on */
pub type FeatureDependencyGraph = HashMap<String, Vec<String>>;
