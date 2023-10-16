use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedTestParamsData {
    ScalaTest(ScalaTestParams),
    ScalaTestSuites(Vec<String>),
    ScalaTestSuitesSelection(ScalaTestSuites),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TestParamsData {
    Named(NamedTestParamsData),
    Other(OtherData),
}

impl TestParamsData {
    pub fn scala_test(data: ScalaTestParams) -> Self {
        Self::Named(NamedTestParamsData::ScalaTest(data))
    }
    pub fn scala_test_suites(data: Vec<String>) -> Self {
        Self::Named(NamedTestParamsData::ScalaTestSuites(data))
    }
    pub fn scala_test_suites_selection(data: ScalaTestSuites) -> Self {
        Self::Named(NamedTestParamsData::ScalaTestSuitesSelection(data))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use insta::assert_compact_json_snapshot;
    use insta::assert_json_snapshot;

    #[test]
    fn test_params_data() {
        assert_json_snapshot!(
           TestParamsData::scala_test(ScalaTestParams::default()),
           @r#"
{
  "dataKind": "scala-test",
  "data": {}
}
   "#
        );

        assert_json_snapshot!(
           TestParamsData::scala_test_suites(Vec::<String>::default()),
           @r#"
{
  "dataKind": "scala-test-suites",
  "data": []
}
   "#
        );

        assert_json_snapshot!(
           TestParamsData::scala_test_suites_selection(ScalaTestSuites::default()),
           @r#"
{
  "dataKind": "scala-test-suites-selection",
  "data": {
    "suites": [],
    "jvmOptions": [],
    "environmentVariables": []
  }
}
   "#
        );

        assert_compact_json_snapshot!(
           TestParamsData::Other(OtherData::default()),
           @r#"{"dataKind": "", "data": null}"#
        );
    }
}
