use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Diagnostic {
    /// The range at which the message applies.
    pub range: Range,
    /// The diagnostic's severity. Can be omitted. If omitted it is up to the
    /// client to interpret diagnostics as error, warning, info or hint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub severity: Option<DiagnosticSeverity>,
    /// The diagnostic's code, which might appear in the user interface.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    /// A human-readable string describing the source of this
    /// diagnostic, e.g. 'typescript' or 'super lint'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// The diagnostic's message.
    pub message: String,
    /// An array of related diagnostic information, e.g. when symbol-names within
    /// a scope collide all definitions can be marked via this property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub related_information: Option<Vec<DiagnosticRelatedInformation>>,
    /// A data entry field that is preserved between a `textDocument/publishDiagnostics` notification
    /// and a `textDocument/codeAction` request.
    #[serde(flatten, skip_serializing_if = "Option::is_none")]
    pub data: Option<DiagnosticData>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;
    use insta::assert_json_snapshot;

    #[test]
    fn diagnostic() {
        assert_json_snapshot!(
           Diagnostic {range: Range::default(), severity: Some(DiagnosticSeverity::default()), code: Some(TEST_STRING.to_string()), source: Some(TEST_STRING.to_string()), message: TEST_STRING.to_string(), related_information: Some(vec![DiagnosticRelatedInformation::default()]), data: Some(DiagnosticData::Other(OtherData::default()))},
           @r#"
{
  "range": {
    "start": {
      "line": 0,
      "character": 0
    },
    "end": {
      "line": 0,
      "character": 0
    }
  },
  "severity": 1,
  "code": "test_string",
  "source": "test_string",
  "message": "test_string",
  "relatedInformation": [
    {
      "location": {
        "uri": "",
        "range": {
          "start": {
            "line": 0,
            "character": 0
          },
          "end": {
            "line": 0,
            "character": 0
          }
        }
      },
      "message": ""
    }
  ],
  "dataKind": "",
  "data": null
}
   "#
        );
    }
}
