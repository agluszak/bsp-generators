use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesItem {
    /// The build target that contains the test classes.
    pub target: BuildTargetIdentifier,
    /// The main class item.
    pub classes: Vec<ScalaMainClass>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_main_classes_item() {
        let test_data = ScalaMainClassesItem {
            target: BuildTargetIdentifier::default(),
            classes: vec![ScalaMainClass::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "target": {
    "uri": ""
  },
  "classes": [
    {
      "class": "",
      "arguments": [],
      "jvmOptions": []
    }
  ]
}
"#);

        test_deserialization(
            r#"{"target": {"uri": ""}, "classes": [{"class": "", "arguments": [], "jvmOptions": []}]}"#,
            &test_data,
        );
    }
}
