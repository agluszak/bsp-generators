use serde::{Deserialize, Serialize};

use crate::*;

/// A unique identifier for a target, can use any URI-compatible encoding as long as it is unique within the workspace.
/// Clients should not infer metadata out of the URI structure such as the path or query parameters, use `BuildTarget` instead.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildTargetIdentifier {
    /// The target's Uri
    pub uri: URI,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn build_target_identifier() {
        assert_json_snapshot!(
           BuildTargetIdentifier {uri: URI::default()},
           @r#"
{
  "uri": ""
}
   "#
        );
    }
}
