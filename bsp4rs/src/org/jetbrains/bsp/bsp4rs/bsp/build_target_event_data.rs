use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedBuildTargetEventData {}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTargetEventData {
    Named(NamedBuildTargetEventData),
    Other(OtherData),
}

impl BuildTargetEventData {}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn build_target_event_data() {
        assert_json_snapshot!(
           BuildTargetEventData::Other(OtherData::default()),
           @r#"
{
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
