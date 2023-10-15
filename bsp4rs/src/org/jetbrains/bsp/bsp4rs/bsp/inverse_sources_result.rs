use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InverseSourcesResult {
    pub targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn inverse_sources_result() {
        assert_json_snapshot!(
           InverseSourcesResult {targets: vec![BuildTargetIdentifier::default()]},
           @r#"
{
  "targets": [
    {
      "uri": ""
    }
  ]
}
   "#
        );
    }
}
