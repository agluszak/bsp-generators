use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CppOptionsResult {
    /// The list of options for each target.
    pub items: Vec<CppOptionsItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn cpp_options_result() {
        let test_data = CppOptionsResult {
            items: vec![CppOptionsItem::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "copts": [],
      "defines": [],
      "linkopts": []
    }
  ]
}
"#);

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "copts": [], "defines": [], "linkopts": []}]}"#,
            &test_data,
        );
    }
}
