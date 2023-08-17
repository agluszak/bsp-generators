use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TestFinishDataKind(pub std::borrow::Cow<'static, str>);
impl TestFinishDataKind {
    pub const fn new(tag: &'static str) -> Self {
        TestFinishDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
