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
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn scala_test_classes_result() {
        assert_json_snapshot!(
           ScalaTestClassesResult {items: vec![ScalaTestClassesItem::default()]},
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
    }
}
