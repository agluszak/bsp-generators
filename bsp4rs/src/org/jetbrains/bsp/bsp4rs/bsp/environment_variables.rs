use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct EnvironmentVariables(pub String);

impl std::ops::Deref for EnvironmentVariables {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for EnvironmentVariables {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for EnvironmentVariables {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}
