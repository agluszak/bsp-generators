use serde::{Deserialize, Serialize};

use crate::*;

/// Structure to capture a description for an error code.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CodeDescription {
    /// An URI to open with more information about the diagnostic error.
    pub href: URI,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn code_description() {
        let test_data = CodeDescription {
            href: URI::default(),
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "href": ""
}
"#);

        test_deserialization(r#"{"href": ""}"#, &test_data);
    }
}
