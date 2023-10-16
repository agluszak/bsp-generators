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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<Identifier>,
    /// The total number of reported errors compiling this target.
    pub errors: i32,
    /// The total number of reported warnings compiling the target.
    pub warnings: i32,
    /// The total number of milliseconds it took to compile the target.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub time: Option<i64>,
    /// The compilation was a noOp compilation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub no_op: Option<bool>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn compile_report() {
        let test_data = CompileReport {
            target: BuildTargetIdentifier::default(),
            origin_id: Some(Identifier::default()),
            errors: TEST_INT,
            warnings: TEST_INT,
            time: Some(TEST_LONG),
            no_op: Some(TEST_BOOL),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "target": {
    "uri": ""
  },
  "originId": "",
  "errors": 1,
  "warnings": 1,
  "time": 2,
  "noOp": true
}
   "#
        );

        test_deserialization(
            r#"{"target": {"uri": ""}, "originId": "", "errors": 1, "warnings": 1, "time": 2, "noOp": true}"#,
            &test_data,
        );
    }
}
