use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RustDependency {
    /// The Package ID of the dependency.
    pub pkg: String,
    /// The name of the dependency's library target.
    /// If this is a renamed dependency, this is the new name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Array of dependency kinds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dep_kinds: Option<Vec<RustDepKindInfo>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn rust_dependency() {
        assert_json_snapshot!(
           RustDependency {pkg: TEST_STRING.to_string(), name: Some(TEST_STRING.to_string()), dep_kinds: Some(vec![RustDepKindInfo::default()])},
           @r#"
{
  "pkg": "test_string",
  "name": "test_string",
  "depKinds": [
    {
      "kind": ""
    }
  ]
}
   "#
        );
    }
}
