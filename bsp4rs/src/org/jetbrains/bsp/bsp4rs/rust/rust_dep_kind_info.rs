use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustDepKindInfo {
    /// The dependency kind.
    pub kind: RustDepKind,
    /// The target platform for the dependency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn rust_dep_kind_info() {
        let test_data = RustDepKindInfo {
            kind: RustDepKind::default(),
            target: Some(TEST_STRING.to_string()),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "kind": "",
  "target": "test_string"
}
   "#
        );

        test_deserialization(r#"{"kind": "", "target": "test_string"}"#, &test_data);
    }
}
