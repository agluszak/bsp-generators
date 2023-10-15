use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PythonOptionsParams {
    pub targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn python_options_params() {
        assert_json_snapshot!(
           PythonOptionsParams {targets: vec![BuildTargetIdentifier::default()]},
           @r#"
{
  "targets": [
    {
      "uri": ""
    }
  ]
}
   "#
        );
    }
}
