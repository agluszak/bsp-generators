use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ScalacOptionsResult {
    pub items: Vec<ScalacOptionsItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn scalac_options_result() {
        let test_data = ScalacOptionsResult {
            items: vec![ScalacOptionsItem::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "options": [],
      "classpath": [],
      "classDirectory": ""
    }
  ]
}
"#);

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "options": [], "classpath": [], "classDirectory": ""}]}"#,
            &test_data,
        );
    }
}
