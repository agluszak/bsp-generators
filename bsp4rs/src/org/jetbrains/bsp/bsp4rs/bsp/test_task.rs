use serde::{Deserialize, Serialize};

use crate::*;

/// The beginning of a testing unit may be signalled to the client with a
/// `build/taskStart` notification. When the testing unit is a build target, the
/// notification's `dataKind` field must be `test-task` and the `data` field must
/// include a `TestTask` object.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestTask {
    pub target: BuildTargetIdentifier,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn test_task() {
        let test_data = TestTask {
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
