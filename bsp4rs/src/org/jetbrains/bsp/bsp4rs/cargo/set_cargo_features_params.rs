use serde::{Deserialize, Serialize};

use crate::*;
use std::collections::BTreeSet;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCargoFeaturesParams {
    /// Package ID for which new features state will be set.
    pub package_id: String,
    /// The list of features to be set as a new state.
    pub features: BTreeSet<Feature>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn set_cargo_features_params() {
        let test_data = SetCargoFeaturesParams {
            package_id: TEST_STRING.to_string(),
            features: BTreeSet::from([Feature::default()]),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "packageId": "test_string",
  "features": [
    ""
  ]
}
"#);

        test_deserialization(
            r#"{"packageId": "test_string", "features": [""]}"#,
            &test_data,
        );
    }
}
