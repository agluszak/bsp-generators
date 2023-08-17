use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TestResultDataKind(pub std::borrow::Cow<'static, str>);
impl TestResultDataKind {
    pub const fn new(tag: &'static str) -> Self {
        TestResultDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
