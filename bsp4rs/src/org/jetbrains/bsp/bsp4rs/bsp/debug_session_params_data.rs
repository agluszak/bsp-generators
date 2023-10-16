use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDebugSessionParamsData {
    ScalaAttachRemote(ScalaAttachRemote),
    ScalaMainClass(ScalaMainClass),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DebugSessionParamsData {
    Named(NamedDebugSessionParamsData),
    Other(OtherData),
}

impl DebugSessionParamsData {
    pub fn scala_attach_remote(data: ScalaAttachRemote) -> Self {
        Self::Named(NamedDebugSessionParamsData::ScalaAttachRemote(data))
    }
    pub fn scala_main_class(data: ScalaMainClass) -> Self {
        Self::Named(NamedDebugSessionParamsData::ScalaMainClass(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;
    use insta::assert_json_snapshot;

    #[test]
    fn debug_session_params_data() {
        assert_json_snapshot!(
           DebugSessionParamsData::scala_attach_remote(ScalaAttachRemote::default()),
           @r#"
{
  "dataKind": "scala-attach-remote",
  "data": {}
}
   "#
        );

        assert_json_snapshot!(
           DebugSessionParamsData::scala_main_class(ScalaMainClass::default()),
           @r#"
{
  "dataKind": "scala-main-class",
  "data": {
    "class": "",
    "arguments": [],
    "jvmOptions": []
  }
}
   "#
        );

        assert_compact_json_snapshot!(
           DebugSessionParamsData::Other(OtherData::default()),
           @r#"{"dataKind": "", "data": null}"#
        );
    }
}
