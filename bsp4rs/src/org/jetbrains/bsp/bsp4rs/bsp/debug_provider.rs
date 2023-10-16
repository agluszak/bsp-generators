use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DebugProvider {
    pub language_ids: Vec<LanguageId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn debug_provider() {
        let test_data = DebugProvider {
            language_ids: vec![LanguageId::default()],
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "languageIds": [
    ""
  ]
}
   "#
        );

        test_deserialization(r#"{"languageIds": [""]}"#, &test_data);
    }
}
