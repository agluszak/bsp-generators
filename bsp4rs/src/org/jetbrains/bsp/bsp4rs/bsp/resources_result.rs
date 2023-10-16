use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesResult {
    pub items: Vec<ResourcesItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn resources_result() {
        let test_data = ResourcesResult {
            items: vec![ResourcesItem::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "resources": []
    }
  ]
}
"#);

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "resources": []}]}"#,
            &test_data,
        );
    }
}
