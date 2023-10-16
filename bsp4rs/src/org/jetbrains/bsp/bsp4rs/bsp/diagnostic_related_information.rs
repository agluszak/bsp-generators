use serde::{Deserialize, Serialize};

use crate::*;

/// Represents a related message and source code location for a diagnostic.
/// This should be used to point to code locations that cause or are related to
/// a diagnostics, e.g when duplicating a symbol in a scope.
#[derive(Clone, Debug, Default, Eq, PartialEq, Hash, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticRelatedInformation {
    /// The location of this related diagnostic information.
    pub location: Location,
    /// The message of this related diagnostic information.
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    use insta::assert_json_snapshot;

    #[test]
    fn diagnostic_related_information() {
        let test_data = DiagnosticRelatedInformation {
            location: Location::default(),
            message: TEST_STRING.to_string(),
        };

        assert_json_snapshot!(
           test_data,
           @r#"
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
  "message": "test_string"
}
   "#
        );

        test_deserialization(
            r#"{"location": {"uri": "", "range": {"start": {"line": 0, "character": 0}, "end": {"line": 0, "character": 0}}}, "message": "test_string"}"#,
            &test_data,
        );
    }
}
