use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsResult {
    /// The list of options for each target.
    pub items: Vec<CppOptionsItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn cpp_options_result() {
        assert_json_snapshot!(
           CppOptionsResult {items: vec![CppOptionsItem::default()]},
           @r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "copts": [],
      "defines": [],
      "linkopts": []
    }
  ]
}
   "#
        );
    }
}
