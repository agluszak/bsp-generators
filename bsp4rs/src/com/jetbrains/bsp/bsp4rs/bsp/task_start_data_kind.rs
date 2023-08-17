use serde::{Deserialize, Serialize};

/** Task start notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field.

There are predefined kinds of objects for compile and test tasks, as described
in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(transparent)]
pub struct TaskStartDataKind(pub std::borrow::Cow<'static, str>);
impl TaskStartDataKind {
    /** `data` field must contain a CompileTask object. */
    pub const COMPILE_TASK: TaskStartDataKind = TaskStartDataKind::new("compile-task");
    /** `data` field must contain a TestStart object. */
    pub const TEST_START: TaskStartDataKind = TaskStartDataKind::new("test-start");
    /** `data` field must contain a TestTask object. */
    pub const TEST_TASK: TaskStartDataKind = TaskStartDataKind::new("test-task");

    pub const fn new(tag: &'static str) -> Self {
        TaskStartDataKind(std::borrow::Cow::Borrowed(tag))
    }
}
