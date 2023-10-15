use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    pub target: BuildTargetIdentifier,
    /// List of resource files.
    pub resources: Vec<URI>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn resources_item() {
        assert_json_snapshot!(
           ResourcesItem {target: BuildTargetIdentifier::default(), resources: vec![URI::default()]},
           @r#"
{
  "target": {
    "uri": ""
  },
  "resources": [
    ""
  ]
}
   "#
        );
    }
}
