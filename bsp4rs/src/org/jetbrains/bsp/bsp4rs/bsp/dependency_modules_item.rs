use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencyModulesItem {
    pub target: BuildTargetIdentifier,
    pub modules: Vec<DependencyModule>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn dependency_modules_item() {
        assert_json_snapshot!(
           DependencyModulesItem {target: BuildTargetIdentifier::default(), modules: vec![DependencyModule::default()]},
           @r#"
{
  "target": {
    "uri": ""
  },
  "modules": [
    {
      "name": "",
      "version": ""
    }
  ]
}
   "#
        );
    }
}
