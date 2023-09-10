use serde::{Deserialize, Serialize};

/** Represents the identifier of a BSP request. */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RequestId(pub String);

impl std::ops::Deref for RequestId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for RequestId {
    fn from(input: String) -> Self {
        RequestId(input)
    }
}

impl From<&str> for RequestId {
    fn from(input: &str) -> Self {
        RequestId(input.to_string())
    }
}
