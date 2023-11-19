use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesResult {
    pub items: Vec<SourcesItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn sources_result() {
        let test_data = SourcesResult {
            items: vec![SourcesItem::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "sources": []
    }
  ]
}
"#);

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "sources": []}]}"#,
            &test_data,
        );
    }
}
