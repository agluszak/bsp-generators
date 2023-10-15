use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanCacheParams {
    /// The build targets to clean.
    pub targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn clean_cache_params() {
        assert_json_snapshot!(
           CleanCacheParams {targets: vec![BuildTargetIdentifier::default()]},
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
