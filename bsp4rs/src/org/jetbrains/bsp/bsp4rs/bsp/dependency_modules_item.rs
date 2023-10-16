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
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn dependency_modules_item() {
        let test_data = DependencyModulesItem {
            target: BuildTargetIdentifier::default(),
            modules: vec![DependencyModule::default()],
        };

        assert_json_snapshot!(test_data,
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
"#);

        test_deserialization(
            r#"{"target": {"uri": ""}, "modules": [{"name": "", "version": ""}]}"#,
            &test_data,
        );
    }
}
