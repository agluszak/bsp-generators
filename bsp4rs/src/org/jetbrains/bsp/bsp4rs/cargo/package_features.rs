use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageFeatures {
    /// The Cargo package identifier.
    pub package_id: String,
    /// The list of build target identifiers assigned to the Cargo package.
    pub targets: Vec<BuildTargetIdentifier>,
    /// The list of available features for the Cargo package.
    pub available_features: FeaturesDependencyGraph,
    /// The list of enabled features for the Cargo package.
    pub enabled_features: BTreeSet<Feature>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn package_features() {
        let test_data = PackageFeatures {
            package_id: TEST_STRING.to_string(),
            targets: vec![BuildTargetIdentifier::default()],
            available_features: FeaturesDependencyGraph::default(),
            enabled_features: BTreeSet::from([Feature::default()]),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "packageId": "test_string",
  "targets": [
    {
      "uri": ""
    }
  ],
  "availableFeatures": {},
  "enabledFeatures": [
    ""
  ]
}
"#);

        test_deserialization(
            r#"{"packageId": "test_string", "targets": [{"uri": ""}], "availableFeatures": {}, "enabledFeatures": [""]}"#,
            &test_data,
        );
    }
}
