use serde::{Deserialize, Serialize};

use crate::*;

#[allow(clippy::large_enum_variant)]
#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "dataKind", content = "data")]
pub enum NamedDiagnosticData {
    Scala(ScalaDiagnostic),
}

#[derive(Clone, Debug, Eq, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum DiagnosticData {
    Named(NamedDiagnosticData),
    Other(OtherData),
}

impl DiagnosticData {
    pub fn scala(data: ScalaDiagnostic) -> Self {
        Self::Named(NamedDiagnosticData::Scala(data))
    }
}

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    use super::*;

    #[test]
    fn diagnostic_data() {
        assert_json_snapshot!(
           DiagnosticData::scala(ScalaDiagnostic::default()),
           @r#"
{
  "dataKind": "scala",
  "data": {}
}
   "#
        );

        assert_json_snapshot!(
           DiagnosticData::Other(OtherData::default()),
           @r#"
{
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
