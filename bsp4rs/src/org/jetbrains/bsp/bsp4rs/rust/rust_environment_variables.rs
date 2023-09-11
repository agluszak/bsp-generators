use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustEnvironmentVariables(pub BTreeMap<String, String>);

impl std::ops::Deref for RustEnvironmentVariables {
    type Target = BTreeMap<String, String>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BTreeMap<String, String>> for RustEnvironmentVariables {
    fn from(input: BTreeMap<String, String>) -> Self {
        RustEnvironmentVariables(input)
    }
}
