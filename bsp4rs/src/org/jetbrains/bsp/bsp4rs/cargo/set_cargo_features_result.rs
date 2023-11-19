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
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn set_cargo_features_result() {
        let test_data = SetCargoFeaturesResult {
            status_code: StatusCode::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "statusCode": 1
}
"#);

        test_deserialization(r#"{"statusCode": 1}"#, &test_data);
    }
}
