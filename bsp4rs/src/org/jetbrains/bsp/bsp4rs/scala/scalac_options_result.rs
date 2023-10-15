use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalacOptionsResult {
    pub items: Vec<ScalacOptionsItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn scalac_options_result() {
        assert_json_snapshot!(
           ScalacOptionsResult {items: vec![ScalacOptionsItem::default()]},
           @r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "options": [],
      "classpath": [],
      "classDirectory": ""
    }
  ]
}
   "#
        );
    }
}
