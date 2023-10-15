use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsResult {
    pub items: Vec<PythonOptionsItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn python_options_result() {
        assert_json_snapshot!(
           PythonOptionsResult {items: vec![PythonOptionsItem::default()]},
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
    }
}
