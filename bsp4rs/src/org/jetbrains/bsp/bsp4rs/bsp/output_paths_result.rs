use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathsResult {
    pub items: Vec<OutputPathsItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn output_paths_result() {
        assert_json_snapshot!(
           OutputPathsResult {items: vec![OutputPathsItem::default()]},
           @r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "outputPaths": []
    }
  ]
}
   "#
        );
    }
}
