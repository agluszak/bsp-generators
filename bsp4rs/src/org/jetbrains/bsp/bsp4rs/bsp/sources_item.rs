use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SourcesItem {
    pub target: BuildTargetIdentifier,
    /// The text documents or and directories that belong to this build target.
    pub sources: Vec<SourceItem>,
    /// The root directories from where source files should be relativized.
    /// Example: ["file://Users/name/dev/metals/src/main/scala"]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<Vec<URI>>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn sources_item() {
        assert_json_snapshot!(
           SourcesItem {target: BuildTargetIdentifier::default(), sources: vec![SourceItem::default()], roots: Some(vec![URI::default()])},
           @r#"
{
  "target": {
    "uri": ""
  },
  "sources": [
    {
      "uri": "",
      "kind": 1,
      "generated": false
    }
  ],
  "roots": [
    ""
  ]
}
   "#
        );
    }
}
