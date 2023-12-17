use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedBuildTargetData {
    Cargo(CargoBuildTarget),
    Cpp(CppBuildTarget),
    Jvm(JvmBuildTarget),
    Python(PythonBuildTarget),
    Sbt(SbtBuildTarget),
    Scala(ScalaBuildTarget),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BuildTargetData {
    Named(NamedBuildTargetData),
    Other(OtherData),
}

impl BuildTargetData {
    pub fn cargo(data: CargoBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Cargo(data))
    }
    pub fn cpp(data: CppBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Cpp(data))
    }
    pub fn jvm(data: JvmBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Jvm(data))
    }
    pub fn python(data: PythonBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Python(data))
    }
    pub fn sbt(data: SbtBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Sbt(data))
    }
    pub fn scala(data: ScalaBuildTarget) -> Self {
        Self::Named(NamedBuildTargetData::Scala(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;
    use insta::assert_json_snapshot;

    use serde_json::json;

    #[test]
    fn build_target_data() {
        assert_json_snapshot!(BuildTargetData::cargo(CargoBuildTarget::default()),
@r#"
{
  "dataKind": "cargo",
  "data": {
    "edition": "",
    "requiredFeatures": []
  }
}
"#);

        assert_json_snapshot!(BuildTargetData::cpp(CppBuildTarget::default()),
@r#"
{
  "dataKind": "cpp",
  "data": {}
}
"#);

        assert_json_snapshot!(BuildTargetData::jvm(JvmBuildTarget::default()),
@r#"
{
  "dataKind": "jvm",
  "data": {}
}
"#);

        assert_json_snapshot!(BuildTargetData::python(PythonBuildTarget::default()),
@r#"
{
  "dataKind": "python",
  "data": {}
}
"#);

        assert_json_snapshot!(BuildTargetData::sbt(SbtBuildTarget::default()),
@r#"
{
  "dataKind": "sbt",
  "data": {
    "sbtVersion": "",
    "autoImports": [],
    "scalaBuildTarget": {
      "scalaOrganization": "",
      "scalaVersion": "",
      "scalaBinaryVersion": "",
      "platform": 1,
      "jars": []
    },
    "children": []
  }
}
"#);

        assert_json_snapshot!(BuildTargetData::scala(ScalaBuildTarget::default()),
@r#"
{
  "dataKind": "scala",
  "data": {
    "scalaOrganization": "",
    "scalaVersion": "",
    "scalaBinaryVersion": "",
    "platform": 1,
    "jars": []
  }
}
"#);

        assert_compact_json_snapshot!(
           BuildTargetData::Other(OtherData { data: json!({}), ..OtherData::default()}),
           @r#"{"dataKind": "", "data": {}}"#
        );
    }
}
