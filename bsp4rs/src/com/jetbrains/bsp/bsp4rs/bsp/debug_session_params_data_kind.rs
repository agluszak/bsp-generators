use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct DebugSessionParamsDataKind(pub std::borrow::Cow<'static, str>);
impl DebugSessionParamsDataKind {
    /** `data` field must contain a ScalaAttachRemote object. */
    pub const SCALA_ATTACH_REMOTE: DebugSessionParamsDataKind =
        DebugSessionParamsDataKind::new("scala-attach-remote");
    /** `data` field must contain a ScalaMainClass object. */
    pub const SCALA_MAIN_CLASS: DebugSessionParamsDataKind =
        DebugSessionParamsDataKind::new("scala-main-class");
    /** `data` field must contain a ScalaTestSuiteClasses object. */
    pub const SCALA_TEST_SUITES: DebugSessionParamsDataKind =
        DebugSessionParamsDataKind::new("scala-test-suites");
    /** `data` field must contain a ScalaTestSuites object. */
    pub const SCALA_TEST_SUITES_SELECTION: DebugSessionParamsDataKind =
        DebugSessionParamsDataKind::new("scala-test-suites-selection");

    pub const fn new(tag: &'static str) -> Self {
        DebugSessionParamsDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
