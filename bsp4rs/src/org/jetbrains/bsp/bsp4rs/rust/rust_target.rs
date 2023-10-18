use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

/// `RustTarget` contains data of the target as defined in Cargo metadata.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustTarget {
    /// The name of the target.
    pub name: String,
    /// Path to the root module of the crate.
    pub crate_root_url: URI,
    /// A target's kind.
    pub kind: RustTargetKind,
    /// Type of output that is produced by a crate during the build process.
    /// The crate type determines how the source code is compiled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub crate_types: Option<Vec<RustCrateType>>,
    /// The Rust edition of the target.
    pub edition: RustEdition,
    /// Whether or not this target has doc tests enabled, and
    /// the target is compatible with doc testing.
    pub doctest: bool,
    /// A sequence of required features.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required_features: Option<BTreeSet<Feature>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn rust_target() {
        let test_data = RustTarget {
            name: TEST_STRING.to_string(),
            crate_root_url: URI::default(),
            kind: RustTargetKind::default(),
            crate_types: Some(vec![RustCrateType::default()]),
            edition: RustEdition::default(),
            doctest: TEST_BOOL,
            required_features: Some(BTreeSet::from([Feature::default()])),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "name": "test_string",
  "crateRootUrl": "",
  "kind": 1,
  "crateTypes": [
    1
  ],
  "edition": "",
  "doctest": true,
  "requiredFeatures": [
    ""
  ]
}
"#);

        test_deserialization(
            r#"{"name": "test_string", "crateRootUrl": "", "kind": 1, "crateTypes": [1], "edition": "", "doctest": true, "requiredFeatures": [""]}"#,
            &test_data,
        );
    }
}
