use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BuildClientCapabilities {
    /// The languages that this client supports.
    /// The ID strings for each language is defined in the LSP.
    /// The server must never respond with build targets for other
    /// languages than those that appear in this list.
    pub language_ids: Vec<LanguageId>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn build_client_capabilities() {
        let test_data = BuildClientCapabilities {
            language_ids: vec![LanguageId::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "languageIds": [
    ""
  ]
}
"#);

        test_deserialization(r#"{"languageIds": [""]}"#, &test_data);
    }
}
