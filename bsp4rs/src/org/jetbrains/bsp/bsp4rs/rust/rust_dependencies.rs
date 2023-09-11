use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeMap;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustDependencies(pub BTreeMap<String, Vec<RustDependency>>);

impl std::ops::Deref for RustDependencies {
    type Target = BTreeMap<String, Vec<RustDependency>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<BTreeMap<String, Vec<RustDependency>>> for RustDependencies {
    fn from(input: BTreeMap<String, Vec<RustDependency>>) -> Self {
        RustDependencies(input)
    }
}
