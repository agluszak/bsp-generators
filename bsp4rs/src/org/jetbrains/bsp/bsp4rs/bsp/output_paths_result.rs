use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OutputPathsResult {
    pub items: Vec<OutputPathsItem>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn output_paths_result() {
        let test_data = OutputPathsResult {
            items: vec![OutputPathsItem::default()],
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
      "outputPaths": []
    }
  ]
}
   "#
        );

        test_deserialization(
            r#"{"items": [{"target": {"uri": ""}, "outputPaths": []}]}"#,
            &test_data,
        );
    }
}
