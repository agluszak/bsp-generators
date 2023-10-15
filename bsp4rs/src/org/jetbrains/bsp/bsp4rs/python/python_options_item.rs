use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsItem {
    pub target: BuildTargetIdentifier,
    /// Attributes added to the interpreter command
    /// For example: -E
    pub interpreter_options: Vec<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn python_options_item() {
        assert_json_snapshot!(
           PythonOptionsItem {target: BuildTargetIdentifier::default(), interpreter_options: vec![TEST_STRING.to_string()]},
           @r#"
{
  "target": {
    "uri": ""
  },
  "interpreterOptions": [
    "test_string"
  ]
}
   "#
        );
    }
}
