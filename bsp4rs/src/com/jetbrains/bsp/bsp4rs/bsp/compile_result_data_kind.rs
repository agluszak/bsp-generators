use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct CompileResultDataKind(pub std::borrow::Cow<'static, str>);
impl CompileResultDataKind {
    pub const fn new(tag: &'static str) -> Self {
        CompileResultDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
