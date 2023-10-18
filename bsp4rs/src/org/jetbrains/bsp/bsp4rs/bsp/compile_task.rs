use serde::{Deserialize, Serialize};

use crate::*;

/// The beginning of a compilation unit may be signalled to the client with a
/// `build/taskStart` notification. When the compilation unit is a build target, the
/// notification's `dataKind` field must be "compile-task" and the `data` field must
/// include a `CompileTask` object:
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileTask {
    pub target: BuildTargetIdentifier,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn compile_task() {
        let test_data = CompileTask {
            target: BuildTargetIdentifier::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "target": {
    "uri": ""
  }
}
"#);

        test_deserialization(r#"{"target": {"uri": ""}}"#, &test_data);
    }
}
