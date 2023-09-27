use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustDependencies(pub String);

impl std::ops::Deref for RustDependencies {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RustDependencies {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for RustDependencies {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}
