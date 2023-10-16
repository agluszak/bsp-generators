use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DependencySourcesParams {
    pub targets: Vec<BuildTargetIdentifier>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn dependency_sources_params() {
        let test_data = DependencySourcesParams {
            targets: vec![BuildTargetIdentifier::default()],
        };

        assert_json_snapshot!(
           test_data,
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

        test_deserialization(r#"{"targets": [{"uri": ""}]}"#, &test_data);
    }
}
