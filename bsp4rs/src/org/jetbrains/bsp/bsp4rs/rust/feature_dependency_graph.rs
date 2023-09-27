use serde::{Deserialize, Serialize};

/// The feature dependency graph is a mapping between
/// feature and the features it turns on
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FeatureDependencyGraph(pub String);

impl std::ops::Deref for FeatureDependencyGraph {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for FeatureDependencyGraph {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for FeatureDependencyGraph {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}
