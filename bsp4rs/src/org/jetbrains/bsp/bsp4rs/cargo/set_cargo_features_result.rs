use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SetCargoFeaturesResult {
    /// The status code of the operation.
    pub status_code: StatusCode,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn set_cargo_features_result() {
        assert_json_snapshot!(
           SetCargoFeaturesResult {status_code: StatusCode::default()},
           @r#"
{
  "statusCode": 1
}
   "#
        );
    }
}
