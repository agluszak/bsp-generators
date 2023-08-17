use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BuildTargetEventDataKind(pub std::borrow::Cow<'static, str>);
impl BuildTargetEventDataKind {
    pub const fn new(tag: &'static str) -> Self {
        BuildTargetEventDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
