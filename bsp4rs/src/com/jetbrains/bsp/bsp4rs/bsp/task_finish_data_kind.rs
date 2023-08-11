use serde::{Deserialize, Serialize};

/** Task finish notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field.

There are predefined kinds of objects for compile and test tasks, as described
in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
#[derive(Debug, PartialEq, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub enum TaskFinishDataKind {
    /** Task finish notifications may contain an arbitrary interface in their `data`
    field. The kind of interface that is contained in a notification must be
    specified in the `dataKind` field.

    There are predefined kinds of objects for compile and test tasks, as described
    in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
    CompileReport,
    /** Task finish notifications may contain an arbitrary interface in their `data`
    field. The kind of interface that is contained in a notification must be
    specified in the `dataKind` field.

    There are predefined kinds of objects for compile and test tasks, as described
    in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
    TestFinish,
    /** Task finish notifications may contain an arbitrary interface in their `data`
    field. The kind of interface that is contained in a notification must be
    specified in the `dataKind` field.

    There are predefined kinds of objects for compile and test tasks, as described
    in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
    TestReport,
}
