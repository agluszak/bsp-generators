use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaTestClassesResult {
    /// An optional id of the request that triggered this result.
    pub items: Vec<ScalaTestClassesItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_test_classes_result() {
        let test_data = ScalaTestClassesResult {
            items: vec![ScalaTestClassesItem::default()],
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "classes": []
    }
  ]
}
   "#
        );

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "classes": []}]}"#,
            &test_data,
        );
    }
}
