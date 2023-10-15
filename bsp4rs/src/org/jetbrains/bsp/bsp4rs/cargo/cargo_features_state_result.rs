use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CargoFeaturesStateResult {
    /// The list of Cargo packages with assigned to them target
    /// identifiers and available features.
    pub packages_features: Vec<PackageFeatures>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn cargo_features_state_result() {
        assert_json_snapshot!(
           CargoFeaturesStateResult {packages_features: vec![PackageFeatures::default()]},
           @r#"
{
  "packagesFeatures": [
    {
      "packageId": "",
      "targets": [],
      "availableFeatures": {},
      "enabledFeatures": []
    }
  ]
}
   "#
        );
    }
}
