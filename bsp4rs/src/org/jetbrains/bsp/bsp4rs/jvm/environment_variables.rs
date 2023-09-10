use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

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
        EnvironmentVariables(input)
    }
}
