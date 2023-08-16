use serde::{Deserialize, Serialize};

/** Task start notifications may contain an arbitrary interface in their `data`
field. The kind of interface that is contained in a notification must be
specified in the `dataKind` field.

There are predefined kinds of objects for compile and test tasks, as described
in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
#[derive(Debug, PartialEq, Clone, Default, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub enum TaskStartDataKind {
    #[default]
    /** Task start notifications may contain an arbitrary interface in their `data`
    field. The kind of interface that is contained in a notification must be
    specified in the `dataKind` field.

    There are predefined kinds of objects for compile and test tasks, as described
    in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
    CompileTask,
    /** Task start notifications may contain an arbitrary interface in their `data`
    field. The kind of interface that is contained in a notification must be
    specified in the `dataKind` field.

    There are predefined kinds of objects for compile and test tasks, as described
    in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
    TestStart,
    /** Task start notifications may contain an arbitrary interface in their `data`
    field. The kind of interface that is contained in a notification must be
    specified in the `dataKind` field.

    There are predefined kinds of objects for compile and test tasks, as described
    in [[bsp#BuildTargetCompile]] and [[bsp#BuildTargetTest]] */
    TestTask,
}
