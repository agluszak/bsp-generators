use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModulesResult {
    pub items: Vec<DependencyModulesItem>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn dependency_modules_result() {
        assert_json_snapshot!(
           DependencyModulesResult {items: vec![DependencyModulesItem::default()]},
           @r#"
{
  "items": [
    {
      "target": {
        "uri": ""
      },
      "modules": []
    }
  ]
}
   "#
        );
    }
}
