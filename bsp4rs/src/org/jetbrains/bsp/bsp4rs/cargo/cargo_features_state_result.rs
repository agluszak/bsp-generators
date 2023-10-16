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
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn cargo_features_state_result() {
        let test_data = CargoFeaturesStateResult {
            packages_features: vec![PackageFeatures::default()],
        };

        assert_json_snapshot!(test_data,
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
"#);

        test_deserialization(
            r#"{"packagesFeatures": [{"packageId": "", "targets": [], "availableFeatures": {}, "enabledFeatures": []}]}"#,
            &test_data,
        );
    }
}
