use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ResourcesItem {
    pub target: BuildTargetIdentifier,
    /// List of resource files.
    pub resources: Vec<URI>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn resources_item() {
        let test_data = ResourcesItem {
            target: BuildTargetIdentifier::default(),
            resources: vec![URI::default()],
        };

        assert_json_snapshot!(test_data,
@r#"
{
  "target": {
    "uri": ""
  },
  "resources": [
    ""
  ]
}
"#);

        test_deserialization(r#"{"target": {"uri": ""}, "resources": [""]}"#, &test_data);
    }
}
