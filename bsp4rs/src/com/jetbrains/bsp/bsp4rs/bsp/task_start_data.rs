use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTaskStartData {
    CompileTask(CompileTask),
    TestStart(TestStart),
    TestTask(TestTask),
}

/** Task start notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field.

There are predefined kinds of objects for compile and test tasks, as described
in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
#[allow(clippy::large_enum_variant)]
#[derive(Debug, PartialEq, Eq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskStartData {
    Named(NamedTaskStartData),
    Other(OtherData),
}

impl TaskStartData {
    pub fn compile_task(data: CompileTask) -> Self {
        TaskStartData::Named(NamedTaskStartData::CompileTask(data))
    }
    pub fn test_start(data: TestStart) -> Self {
        TaskStartData::Named(NamedTaskStartData::TestStart(data))
    }
    pub fn test_task(data: TestTask) -> Self {
        TaskStartData::Named(NamedTaskStartData::TestTask(data))
    }
}
