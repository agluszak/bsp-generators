use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RustCfgOptions(pub String);

impl std::ops::Deref for RustCfgOptions {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RustCfgOptions {
    fn from(input: String) -> Self {
        Self(input)
    }
}

impl From<&str> for RustCfgOptions {
    fn from(input: &str) -> Self {
        Self(input.to_string())
    }
}
