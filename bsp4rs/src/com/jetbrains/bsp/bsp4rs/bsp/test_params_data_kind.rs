use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TestParamsDataKind(pub std::borrow::Cow<'static, str>);
impl TestParamsDataKind {
    /** `data` field must contain a ScalaTestParams object. */
    pub const SCALA_TEST: TestParamsDataKind = TestParamsDataKind::new("scala-test");

    pub const fn new(tag: &'static str) -> Self {
        TestParamsDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
