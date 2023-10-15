use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct JavacOptionsResult {
    pub items: Vec<JavacOptionsItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn javac_options_result() {
        assert_json_snapshot!(
           JavacOptionsResult {items: vec![JavacOptionsItem::default()]},
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
