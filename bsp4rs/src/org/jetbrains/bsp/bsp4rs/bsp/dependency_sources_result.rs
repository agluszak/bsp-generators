use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencySourcesResult {
    pub items: Vec<DependencySourcesItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn dependency_sources_result() {
        assert_json_snapshot!(
           DependencySourcesResult {items: vec![DependencySourcesItem::default()]},
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
