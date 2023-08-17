use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTaskFinishData {
    CompileReport(CompileReport),
    TestFinish(TestFinish),
    TestReport(TestReport),
}

/** Task finish notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field.

There are predefined kinds of objects for compile and test tasks, as described
in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TaskFinishData {
    Named(NamedTaskFinishData),
    Other(OtherData),
}

impl TaskFinishData {
    pub fn compile_report(data: CompileReport) -> Self {
        TaskFinishData::Named(NamedTaskFinishData::CompileReport(data))
    }
    pub fn test_finish(data: TestFinish) -> Self {
        TaskFinishData::Named(NamedTaskFinishData::TestFinish(data))
    }
    pub fn test_report(data: TestReport) -> Self {
        TaskFinishData::Named(NamedTaskFinishData::TestReport(data))
    }
}
