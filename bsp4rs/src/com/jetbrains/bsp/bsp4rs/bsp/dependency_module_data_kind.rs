use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DependencyModuleDataKind(pub std::borrow::Cow<'static, str>);
impl DependencyModuleDataKind {
    /** `data` field must contain a MavenDependencyModule object. */
    pub const MAVEN: DependencyModuleDataKind = DependencyModuleDataKind::new("maven");

    pub const fn new(tag: &'static str) -> Self {
        DependencyModuleDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
