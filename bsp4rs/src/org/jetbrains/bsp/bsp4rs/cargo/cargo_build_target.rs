use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

/// `CargoBuildTarget` is a basic data structure that contains
/// cargo-specific metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoBuildTarget {
    pub edition: RustEdition,
    pub required_features: BTreeSet<Feature>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn cargo_build_target() {
        assert_json_snapshot!(
           CargoBuildTarget {edition: RustEdition::default(), required_features: BTreeSet::from([Feature::default()])},
           @r#"
{
  "edition": "",
  "requiredFeatures": [
    ""
  ]
}
   "#
        );
    }
}
