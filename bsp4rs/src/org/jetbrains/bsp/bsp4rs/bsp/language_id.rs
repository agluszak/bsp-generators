use serde::{Deserialize, Serialize};

/** Language IDs are defined here
https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#textDocumentItem */
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(transparent)]
pub struct LanguageId(pub String);

impl std::ops::Deref for LanguageId {
    type Target = String;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<String> for LanguageId {
    fn from(input: String) -> Self {
        LanguageId(input)
    }
}

impl From<&str> for LanguageId {
    fn from(input: &str) -> Self {
        LanguageId(input.to_string())
    }
}
