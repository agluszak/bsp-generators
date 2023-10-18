use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalaMainClassesResult {
    pub items: Vec<ScalaMainClassesItem>,
    /// An optional id of the request that triggered this result.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub origin_id: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scala_main_classes_result() {
        let test_data = ScalaMainClassesResult {
            items: vec![ScalaMainClassesItem::default()],
            origin_id: Some(TEST_STRING.to_string()),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "classes": []
    }
  ],
  "originId": "test_string"
}
"#);

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "classes": []}], "originId": "test_string"}"#,
            &test_data,
        );
    }
}
