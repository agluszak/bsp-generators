use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesResult {
    pub items: Vec<ResourcesItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn resources_result() {
        assert_json_snapshot!(
           ResourcesResult {items: vec![ResourcesItem::default()]},
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
   "#
        );
    }
}
