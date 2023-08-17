use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct RunParamsDataKind(pub std::borrow::Cow<'static, str>);
impl RunParamsDataKind {
    /** `data` field must contain a ScalaMainClass object. */
    pub const SCALA_MAIN_CLASS: RunParamsDataKind = RunParamsDataKind::new("scala-main-class");

    pub const fn new(tag: &'static str) -> Self {
        RunParamsDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
