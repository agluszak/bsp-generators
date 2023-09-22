use serde::{Deserialize, Serialize};

use crate::*;

/// The completion of a compilation task should be signalled with a
/// `build/taskFinish` notification. When the compilation unit is a build target,
/// the notification's `dataKind` field must be `compile-report` and the `data`
/// field must include a `CompileReport` object:
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileReport {
    /// The build target that was compiled.
    pub target: BuildTargetIdentifier,
    /// An optional request id to know the origin of this report.
    #[deprecated(note = "Use the field in TaskFinishParams instead")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// The total number of reported errors compiling this target.
    pub errors: i32,
    /// The total number of reported warnings compiling the target.
    pub warnings: i32,
    /// The total number of milliseconds it took to compile the target.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// The compilation was a noOp compilation.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_op: Option<bool>,
}
