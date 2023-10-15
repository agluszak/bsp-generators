use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CompileProvider {
    pub language_ids: Vec<LanguageId>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn compile_provider() {
        assert_json_snapshot!(
           CompileProvider {language_ids: vec![LanguageId::default()]},
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
