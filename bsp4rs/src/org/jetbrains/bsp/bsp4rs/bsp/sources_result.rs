use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesResult {
    pub items: Vec<SourcesItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn sources_result() {
        assert_json_snapshot!(
           SourcesResult {items: vec![SourcesItem::default()]},
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
   "#
        );
    }
}
