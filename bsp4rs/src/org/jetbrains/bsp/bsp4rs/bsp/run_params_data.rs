use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedRunParamsData {
    ScalaMainClass(ScalaMainClass),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum RunParamsData {
    Named(NamedRunParamsData),
    Other(OtherData),
}

impl RunParamsData {
    pub fn scala_main_class(data: ScalaMainClass) -> Self {
        Self::Named(NamedRunParamsData::ScalaMainClass(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;
    use insta::assert_json_snapshot;

    #[test]
    fn run_params_data() {
        assert_json_snapshot!(RunParamsData::scala_main_class(ScalaMainClass::default()),
@r#"
{
  "dataKind": "scala-main-class",
  "data": {
    "class": "",
    "arguments": [],
    "jvmOptions": []
  }
}
"#);

        assert_compact_json_snapshot!(
           RunParamsData::Other(OtherData::default()),
           @r#"{"dataKind": "", "data": null}"#
        );
    }
}
