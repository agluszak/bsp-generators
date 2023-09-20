use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

/// Map representing the environment variables used in BSP extensions.
/// Each key represents an environment variable name and each value represents the corresponding value to be set.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EnvironmentVariables(pub BTreeMap<String, String>);

impl std::ops::Deref for EnvironmentVariables {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BTreeMap<String, String>> for EnvironmentVariables {
    fn from(input: BTreeMap<String, String>) -> Self {
        Self(input)
    }
}
