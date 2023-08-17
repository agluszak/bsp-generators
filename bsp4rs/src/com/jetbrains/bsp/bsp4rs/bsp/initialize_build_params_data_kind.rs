use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct InitializeBuildParamsDataKind(pub std::borrow::Cow<'static, str>);
impl InitializeBuildParamsDataKind {
    pub const fn new(tag: &'static str) -> Self {
        InitializeBuildParamsDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
