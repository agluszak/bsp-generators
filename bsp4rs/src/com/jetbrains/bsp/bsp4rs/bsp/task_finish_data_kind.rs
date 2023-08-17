use serde::{Deserialize, Serialize};

/** Task finish notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field.

There are predefined kinds of objects for compile and test tasks, as described
in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskFinishDataKind(pub std::borrow::Cow<'static, str>);
impl TaskFinishDataKind {
    /** `data` field must contain a CompileReport object. */
    pub const COMPILE_REPORT: TaskFinishDataKind = TaskFinishDataKind::new("compile-report");
    /** `data` field must contain a TestFinish object. */
    pub const TEST_FINISH: TaskFinishDataKind = TaskFinishDataKind::new("test-finish");
    /** `data` field must contain a TestReport object. */
    pub const TEST_REPORT: TaskFinishDataKind = TaskFinishDataKind::new("test-report");

    pub const fn new(tag: &'static str) -> Self {
        TaskFinishDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
