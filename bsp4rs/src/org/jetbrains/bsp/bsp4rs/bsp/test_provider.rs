use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TestProvider {
    pub language_ids: Vec<LanguageId>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn test_provider() {
        assert_json_snapshot!(
           TestProvider {language_ids: vec![LanguageId::default()]},
           @r#"
{
  "languageIds": [
    ""
  ]
}
   "#
        );
    }
}
