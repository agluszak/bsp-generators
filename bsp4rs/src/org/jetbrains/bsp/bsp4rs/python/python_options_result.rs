use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsResult {
    pub items: Vec<PythonOptionsItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn python_options_result() {
        let test_data = PythonOptionsResult {
            items: vec![PythonOptionsItem::default()],
        };

        assert_json_snapshot!(
           test_data,
           @r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "interpreterOptions": []
    }
  ]
}
   "#
        );

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "interpreterOptions": []}]}"#,
            &test_data,
        );
    }
}
