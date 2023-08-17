use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitializeBuildResultDataKind(pub std::borrow::Cow<'static, str>);
impl InitializeBuildResultDataKind {
    pub const fn new(tag: &'static str) -> Self {
        InitializeBuildResultDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
