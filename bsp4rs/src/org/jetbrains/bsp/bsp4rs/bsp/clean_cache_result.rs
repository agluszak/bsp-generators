use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CleanCacheResult {
    /// Optional message to display to the user.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Indicates whether the clean cache request was performed or not.
    pub cleaned: bool,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn clean_cache_result() {
        assert_json_snapshot!(
           CleanCacheResult {message: Some(TEST_STRING.to_string()), cleaned: TEST_BOOL},
           @r#"
{
  "message": "test_string",
  "cleaned": true
}
   "#
        );
    }
}
